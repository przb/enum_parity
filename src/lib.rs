//! This crate exposes the [`macro@bit_parity`] macro to enforce a given bit parity
//!
//! # Examples
//!
//! ## Even Bit Parity
//! ```
//! use enum_parity::bit_parity;
//!
//! #[bit_parity(even)]
//! #[repr(u8)]
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
//! #[bit_parity(odd)]
//! #[repr(u8)]
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

use std::{collections::HashSet, fmt::Display, str::FromStr};

use bit_par_iter::{BitParityIter, IntegerParity};
use darling::{FromAttributes, FromMeta};
use int_repr::IntRepr;
use proc_macro2::TokenStream;
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
                Parity::Even => "even",
                Parity::Odd => "odd",
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
    explicit_discriminants: &HashSet<N>,
) -> syn::Result<N>
where
    N: IntegerParity + Eq + std::hash::Hash,
{
    // TODO not a huge fan of the control flow in this function...
    for next_val in bpi {
        if explicit_discriminants.contains(&next_val) {
            continue;
        } else {
            return Ok(next_val);
        }
    }

    // if we got out of the for loop without returning, then we ran out of discriminants
    return Err(syn::Error::new_spanned(
        &variant,
        format!(
            "ran out of discriminant values for `{}` repr type",
            ctx.repr
        ),
    ));
}

fn generic_expand<T>(ctx: &Ctx, mut enum_item: ItemEnum) -> syn::Result<TokenStream>
where
    T: IntegerParity + darling::ToTokens + FromStr + Eq + std::hash::Hash + std::fmt::Debug + Ord,
    T::Err: Display,
{
    // iterate through all the enum variants, and validate all the explicit discriminants
    let explicit_discriminants = enum_item
        .variants
        .iter()
        .filter_map(|variant| {
            variant
                .discriminant
                .clone()
                .map(|disc| parse_discriminant::<T>(ctx, disc))
        })
        .collect::<syn::Result<HashSet<T>>>()?;

    let mut bpi = BitParityIter::<T>::new(ctx.parity);
    for variant in &mut enum_item.variants {
        let next_disc = match variant.discriminant.clone() {
            Some(disc) => {
                let next_disc = parse_discriminant(ctx, disc)?;

                // update the bpi iter so that it only uses values greater than the last explicit discriminants
                // this behavior echos the default rust discriminant behavior
                while let Some(val) = bpi.next()
                    && val < next_disc
                {}

                next_disc
            }
            None => next_discriminant(ctx, &mut bpi, variant, &explicit_discriminants)?,
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
/// See the [crate-level](crate) docs for examples.
///
/// # Macro Parameters
/// The only accepted parameters to the macro is `odd` and `even`
///
/// # Examples
///
/// In order to use even parity for enum discriminants:
/// ```skip
/// #[bit_parity(even)]
/// ```
///
/// In order to use odd parity for enum discriminants:
/// ```skip
/// #[bit_parity(odd)]
/// ```
#[proc_macro_attribute]
pub fn bit_parity(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as BitParityArgs);
    let enum_item = parse_macro_input!(input as ItemEnum);

    try_expand(&args, enum_item).map_or_else(|e| e.into_compile_error().into(), Into::into)
}
