//! This crate exposes the [`bit_parity`] macro to enforce a given bit parity
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

use bit_par_iter::{BitParityIter, IntegerParity};
use darling::{FromAttributes, FromMeta};
use int_repr::IntRepr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse_macro_input};

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
}

fn generic_expand<T: IntegerParity + darling::ToTokens>(
    repr: IntRepr,
    args: &BitParityArgs,
    mut enum_item: ItemEnum,
) -> syn::Result<TokenStream> {
    let mut bpi = BitParityIter::<T>::new(matches!(args.parity, Parity::Even));
    for variant in &mut enum_item.variants {
        if variant.discriminant.is_some() {
            return Err(syn::Error::new_spanned(
                &variant,
                "explicit discriminants are unsupported",
            ));
        }

        let next_disc = bpi.next().ok_or_else(|| {
            syn::Error::new_spanned(
                &variant,
                format!("ran out of discriminant values for `{repr}` repr type",),
            )
        })?;

        variant.discriminant = Some((syn::token::Eq::default(), syn::parse_quote!(#next_disc)));
    }

    Ok(quote! {#enum_item})
}
fn specialize_expand(
    repr: IntRepr,
    args: &BitParityArgs,
    enum_item: ItemEnum,
) -> syn::Result<TokenStream> {
    match repr {
        IntRepr::U8 => generic_expand::<u8>(repr, args, enum_item),
        IntRepr::U16 => generic_expand::<u16>(repr, args, enum_item),
        IntRepr::U32 => generic_expand::<u32>(repr, args, enum_item),
        IntRepr::U64 => generic_expand::<u64>(repr, args, enum_item),
        IntRepr::U128 => generic_expand::<u128>(repr, args, enum_item),
        IntRepr::Usize => generic_expand::<usize>(repr, args, enum_item),
        IntRepr::I8 => generic_expand::<i8>(repr, args, enum_item),
        IntRepr::I16 => generic_expand::<i16>(repr, args, enum_item),
        IntRepr::I32 => generic_expand::<i32>(repr, args, enum_item),
        IntRepr::I64 => generic_expand::<i64>(repr, args, enum_item),
        IntRepr::I128 => generic_expand::<i128>(repr, args, enum_item),
        IntRepr::Isize => generic_expand::<isize>(repr, args, enum_item),
    }
}

fn try_expand(args: &BitParityArgs, enum_item: ItemEnum) -> syn::Result<TokenStream> {
    let repr = IntRepr::from_attributes(&enum_item.attrs)?;
    specialize_expand(repr, args, enum_item)
}

#[proc_macro_attribute]
pub fn bit_parity(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as BitParityArgs);
    let enum_item = parse_macro_input!(input as ItemEnum);

    try_expand(&args, enum_item).map_or_else(|e| e.into_compile_error().into(), Into::into)
}
