mod bit_par_iter;

use std::str::FromStr;

use bit_par_iter::{BitParityIter, IntegerParity};
use darling::{FromAttributes, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse_macro_input};

#[derive(Copy, Clone, Debug)]
enum IntRepr {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}

impl FromStr for IntRepr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "u8" => Ok(Self::U8),
            "u16" => Ok(Self::U16),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "u128" => Ok(Self::U128),
            "usize" => Ok(Self::Usize),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "i128" => Ok(Self::I128),
            "isize" => Ok(Self::Isize),
            _ => Err(()),
        }
    }
}

impl FromAttributes for IntRepr {
    fn from_attributes(attrs: &[syn::Attribute]) -> darling::Result<Self> {
        let mut int_repr = None;
        let repr_attr = attrs
            .iter()
            .find(|a| a.path().is_ident("repr"))
            .ok_or_else(|| darling::Error::custom("unable to find `repr` attribute"))?;

        repr_attr.parse_nested_meta(|m| {
            let repr_type = m
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&m.path, "missing `repr` type"))?
                .to_string();
            let ir = IntRepr::from_str(&repr_type)
                .map_err(|()| syn::Error::new_spanned(m.path, "unsupported `repr` type"))?;
            int_repr = Some(ir);

            Ok(())
        })?;

        int_repr.ok_or_else(|| darling::Error::custom("unable to find a valid `repr` attribute"))
    }
}

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
    args: BitParityArgs,
    mut enum_item: ItemEnum,
) -> syn::Result<TokenStream> {
    let mut bpi = BitParityIter::<T>::new(matches!(args.parity, Parity::Even));
    for variant in enum_item.variants.iter_mut() {
        if variant.discriminant.is_some() {
            return Err(syn::Error::new_spanned(
                &variant,
                "explicit discriminants are not supported",
            ));
        } else {
            let next_disc = bpi.next().ok_or_else(|| {
                syn::Error::new_spanned(&variant, "ran out of discriminant values for repr type")
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
        IntRepr::U8 => generic_expand::<u8>(args, enum_item),
        IntRepr::U16 => generic_expand::<u16>(args, enum_item),
        IntRepr::U32 => generic_expand::<u32>(args, enum_item),
        IntRepr::U64 => generic_expand::<u64>(args, enum_item),
        IntRepr::U128 => generic_expand::<u128>(args, enum_item),
        IntRepr::Usize => generic_expand::<usize>(args, enum_item),
        IntRepr::I8 => generic_expand::<i8>(args, enum_item),
        IntRepr::I16 => generic_expand::<i16>(args, enum_item),
        IntRepr::I32 => generic_expand::<i32>(args, enum_item),
        IntRepr::I64 => generic_expand::<i64>(args, enum_item),
        IntRepr::I128 => generic_expand::<i128>(args, enum_item),
        IntRepr::Isize => generic_expand::<isize>(args, enum_item),
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
