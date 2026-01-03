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

use std::{fmt::Display, str::FromStr};

use bit_par_iter::{BitParityIter, IntegerParity};
use darling::{FromAttributes, FromMeta};
use int_repr::IntRepr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ItemEnum, parse_macro_input, spanned::Spanned};

#[derive(Copy, Clone, Debug, FromMeta)]
enum Parity {
    Even,
    Odd,
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
    #[expect(unused)]
    allow_explicit_overrides: bool,
}

fn discrimin_val<N>(ctx: &Ctx, expr: Expr) -> syn::Result<N>
where
    N: IntegerParity + darling::ToTokens + FromStr,
    N::Err: Display,
{
    let x = if let Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(litint),
        ..
    }) = expr.clone()
    {
        litint.base10_parse::<N>().ok()
    } else {
        None
    };
    x.ok_or(syn::Error::new(expr.span(), "invalid discriminant"))
}

fn generic_expand<T>(ctx: &Ctx, mut enum_item: ItemEnum) -> syn::Result<TokenStream>
where
    T: IntegerParity + darling::ToTokens + FromStr,
    T::Err: Display,
{
    let mut bpi = BitParityIter::<T>::new(ctx.parity);
    for variant in &mut enum_item.variants {
        let next_disc = match variant.discriminant.clone() {
            Some((_eq_tok, expr)) => discrimin_val(ctx, expr)?,
            None => bpi.next().ok_or_else(|| {
                syn::Error::new_spanned(
                    &variant,
                    format!(
                        "ran out of discriminant values for `{}` repr type",
                        ctx.repr
                    ),
                )
            })?,
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
