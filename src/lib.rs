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
    args: BitParityArgs,
    mut enum_item: ItemEnum,
) -> syn::Result<TokenStream> {
    let mut bpi = BitParityIter::<T>::new(matches!(args.parity, Parity::Even));
    for variant in enum_item.variants.iter_mut() {
        if variant.discriminant.is_some() {
            return Err(syn::Error::new_spanned(
                &variant,
                "explicit discriminants are unsupported",
            ));
        } else {
            let next_disc = bpi.next().ok_or_else(|| {
                syn::Error::new_spanned(
                    &variant,
                    format!("ran out of discriminant values for `{repr}` repr type",),
                )
            })?;

            variant.discriminant = Some((syn::token::Eq::default(), syn::parse_quote!(#next_disc)));
        }
    }

    Ok(quote! {#enum_item})
}
fn specialize_expand(
    repr: IntRepr,
    args: BitParityArgs,
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

fn try_expand(args: BitParityArgs, enum_item: ItemEnum) -> syn::Result<TokenStream> {
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

    try_expand(args, enum_item)
        .map(Into::into)
        .unwrap_or_else(|e| e.into_compile_error().into())
}
