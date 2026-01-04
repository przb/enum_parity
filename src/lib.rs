//! This crate exposes the [`macro@bit_parity`] macro to enforce a given bit parity
//!
//! # Motivation
//!
//! Without bit parity, erroneous/random bit flips can lead to unexpected behavior.
//!
//! ## Without Bit Parity
//! ```
//! use enum_parity::bit_parity;
//! use serde_repr::{Serialize_repr, Deserialize_repr};
//!
//! #[repr(u8)]
//! # #[derive(Debug, Eq, PartialEq)]
//! // `serialize_repr` must be used, because `serde` uses enum indexes for binary serializations
//! #[derive(Serialize_repr, Deserialize_repr)]
//! enum Foo { A, B, C, D }
//!
//! let val = Foo::A;
//! let mut serialized_val = postcard::to_allocvec(&val).unwrap();
//!
//! // *Random bit flip*
//! serialized_val[0] |= 0x01;
//!
//! // This successfully deserializes, but is the incorrect value!
//! let new_val: Foo = postcard::from_bytes(&serialized_val).unwrap();
//! assert_eq!(new_val, Foo::B);
//! assert_ne!(val, new_val);
//! ```
//!
//! ## With Bit Parity
//! ```
//! # use enum_parity::bit_parity;
//! # use serde_repr::{Serialize_repr, Deserialize_repr};
//! #[repr(u8)]
//! #[bit_parity(even)] // using bit parity!
//! # #[derive(Debug, Eq, PartialEq)]
//! #[derive(Serialize_repr, Deserialize_repr)]
//! enum Foo { A, B, C, D }
//! let val = Foo::A;
//! let mut serialized_val = postcard::to_allocvec(&val).unwrap();
//!
//! // *Random bit flip*
//! serialized_val[0] |= 0x01;
//!
//! // This fails to deserialize
//! let new_par_err: postcard::Result<Foo> = postcard::from_bytes(&serialized_val);
//! assert_eq!(new_par_err, Err(postcard::Error::SerdeDeCustom));
//! ```
//! # Examples
//!
//! ## Even Bit Parity
//! ```
//! use enum_parity::bit_parity;
//!
//! #[repr(u8)]
//! #[bit_parity(even)]
//! pub enum EvenSample {
//!     Foo,
//!     Bar,
//!     Baz,
//!     Quo,
//! }
//!
//! assert_eq!(EvenSample::Foo as u8, 0x00);
//! assert_eq!(EvenSample::Bar as u8, 0x03);
//! assert_eq!(EvenSample::Baz as u8, 0x05);
//! assert_eq!(EvenSample::Quo as u8, 0x06);
//! ```
//!
//! ## Odd Bit Parity
//! ```
//! use enum_parity::bit_parity;
//!
//! #[repr(u8)]
//! #[bit_parity(odd)]
//! pub enum OddSample {
//!     Lorem,
//!     Ipsum,
//!     Dolor,
//!     Sit,
//! }
//!
//! assert_eq!(OddSample::Lorem as u8, 0x01);
//! assert_eq!(OddSample::Ipsum as u8, 0x02);
//! assert_eq!(OddSample::Dolor as u8, 0x04);
//! assert_eq!(OddSample::Sit as u8, 0x07);
//! ```

mod bit_par_iter;
mod int_repr;

use std::{collections::HashMap, fmt::Display, str::FromStr};

use bit_par_iter::{BitParityIter, IntegerParity};
use darling::{FromAttributes, FromMeta};
use int_repr::IntRepr;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Expr, ItemEnum, Variant, parse_macro_input, spanned::Spanned};

#[derive(Copy, Clone, Debug, FromMeta)]
enum Parity {
    Even,
    Odd,
}

// TODO could probably get rid of this for some provided method from darling?
impl Display for Parity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Even => "even",
                Self::Odd => "odd",
            }
        )
    }
}

#[derive(Debug, Clone, FromMeta)]
#[darling(derive_syn_parse)]
struct BitParityArgs {
    #[darling(flatten)]
    parity: Parity,
    #[darling(default)]
    allow_explicit_overrides: bool,
}

struct Ctx {
    repr: IntRepr,
    parity: Parity,
    allow_explicit_overrides: bool,
}

fn parse_discriminant<N>(ctx: &Ctx, (_eq_tok, expr): (syn::token::Eq, Expr)) -> syn::Result<N>
where
    N: IntegerParity + darling::ToTokens + FromStr,
    N::Err: Display,
{
    let Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(lit),
        ..
    }) = expr.clone()
    else {
        // the expression was not a valid literal
        return Err(syn::Error::new(
            expr.span(),
            "Invalid or unsupported enum discriminant value",
        ));
    };

    let lit = lit.base10_parse::<N>()?;

    if lit.has_parity(ctx.parity) || ctx.allow_explicit_overrides {
        Ok(lit)
    } else {
        Err(syn::Error::new(
            expr.span(),
            format!(
                "explicit discriminant does not have `{}` parity",
                ctx.parity,
            ),
        ))
    }
}

fn next_discriminant<N>(
    ctx: &Ctx,
    bpi: &mut BitParityIter<N>,
    variant: &Variant,
    explicit_discriminants: &HashMap<N, Span>,
) -> syn::Result<N>
where
    N: IntegerParity + Eq + std::hash::Hash,
{
    // TODO not a huge fan of the control flow in this function...
    if let Some(next_val) = bpi.next() {
        if let Some(span) = explicit_discriminants.get(&next_val) {
            let mut err = syn::Error::new(*span, "previous assignment here");

            err.combine(syn::Error::new(
                variant.span(),
                "discriminant value is already assigned",
            ));

            return Err(err);
        }
        return Ok(next_val);
    }

    // if we got out of the for loop without returning, then we ran out of discriminants
    Err(syn::Error::new_spanned(
        variant,
        format!(
            "ran out of discriminant values for `{}` repr type",
            ctx.repr
        ),
    ))
}

fn generic_expand<T>(ctx: &Ctx, mut enum_item: ItemEnum) -> syn::Result<TokenStream>
where
    T: IntegerParity + darling::ToTokens + FromStr + Eq + std::hash::Hash + std::fmt::Debug + Ord,
    T::Err: Display,
{
    // iterate through all the enum variants, and validate all the explicit discriminants
    let mut explicit_discriminants = enum_item
        .variants
        .iter()
        .filter_map(|variant| {
            variant
                .discriminant
                .clone()
                .map(|disc| parse_discriminant::<T>(ctx, disc).map(|val| (val, variant.span())))
        })
        .collect::<syn::Result<HashMap<T, Span>>>()?;

    let mut bpi = BitParityIter::<T>::new(ctx.parity);
    for variant in &mut enum_item.variants {
        let next_disc = if let Some(disc) = variant.discriminant.clone() {
            let next_disc = parse_discriminant(ctx, disc)?;

            bpi.set_override(next_disc);

            next_disc
        } else {
            let next_disc = next_discriminant(ctx, &mut bpi, variant, &explicit_discriminants)?;

            explicit_discriminants.insert(next_disc, variant.span());
            next_disc
        };

        variant.discriminant = Some((syn::token::Eq::default(), syn::parse_quote!(#next_disc)));
    }

    Ok(quote! {#enum_item})
}
fn specialize_expand(ctx: &Ctx, enum_item: ItemEnum) -> syn::Result<TokenStream> {
    match ctx.repr {
        IntRepr::U8 => generic_expand::<u8>(ctx, enum_item),
        IntRepr::U16 => generic_expand::<u16>(ctx, enum_item),
        IntRepr::U32 => generic_expand::<u32>(ctx, enum_item),
        IntRepr::U64 => generic_expand::<u64>(ctx, enum_item),
        IntRepr::U128 => generic_expand::<u128>(ctx, enum_item),
        IntRepr::Usize => generic_expand::<usize>(ctx, enum_item),
        IntRepr::I8 => generic_expand::<i8>(ctx, enum_item),
        IntRepr::I16 => generic_expand::<i16>(ctx, enum_item),
        IntRepr::I32 => generic_expand::<i32>(ctx, enum_item),
        IntRepr::I64 => generic_expand::<i64>(ctx, enum_item),
        IntRepr::I128 => generic_expand::<i128>(ctx, enum_item),
        IntRepr::Isize => generic_expand::<isize>(ctx, enum_item),
    }
}

fn try_expand(args: &BitParityArgs, enum_item: ItemEnum) -> syn::Result<TokenStream> {
    let repr = IntRepr::from_attributes(&enum_item.attrs)?;
    let ctx = Ctx {
        repr,
        parity: args.parity,
        allow_explicit_overrides: args.allow_explicit_overrides,
    };
    specialize_expand(&ctx, enum_item)
}

/// An attribute macro for enums that enforces discriminant bit parity
///
/// See the [crate-level](crate) docs for more examples.
///
/// # Macro Parameters
/// - `even` enforces even bit parity
/// - `odd` enforces odd bit parity
/// - `allow_explicit_overrides` accepts a boolean. It causes an explicit enum discriminant that does not match the given bit partity to:
///   - If `true`, successfully compile.
///   - If `false`, fail to compile
///
///   `allow_explicit_overrides` is optional, and defaults to `false`.
///
/// # Examples
///
/// ## Simple Usage
/// In order to use even parity for enum discriminants:
/// ```skip
/// #[bit_parity(even)]
/// ```
///
/// In order to use odd parity for enum discriminants:
/// ```skip
/// #[bit_parity(odd)]
/// ```
///
/// ## Explicit Discriminant Values
/// By default, assigning a value to an enum discriminant that does not have the matching bit parity fails to compile
/// ```compile_fail
/// # use enum_parity::bit_parity;
/// #[repr(u8)]
/// #[bit_parity(even)]
/// enum Foo {
///   // this fails to compile, because `0x01` does not have even bit parity
///   A = 0x01,
///   B,
///   C,
/// }
/// ```
///
/// If you want to allow explicit discriminants that do not match the given bit parity, add `allow_explicit_overrides`
/// ```
/// # use enum_parity::bit_parity;
/// #[repr(u8)]
/// #[bit_parity(even, allow_explicit_overrides = true)]
/// enum Foo {
///   // `0x01` does not have even bit parity, but it is allowed from the `allow_explicit_overrides` parameter
///   A = 0x01,
///   B,
///   C,
/// }
///
/// assert_eq!(Foo::A as u8, 0x01);
/// // `B` and `C` will have even bit parity
/// assert_eq!(Foo::B as u8, 0x03);
/// assert_eq!(Foo::C as u8, 0x05);
/// ```
///
#[proc_macro_attribute]
pub fn bit_parity(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as BitParityArgs);
    let enum_item = parse_macro_input!(input as ItemEnum);

    try_expand(&args, enum_item).map_or_else(|e| e.into_compile_error().into(), Into::into)
}
