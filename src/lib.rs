mod bit_par_iter;

use bit_par_iter::BitParityIter;
use darling::FromMeta;
use proc_macro2::TokenStream;
use syn::{ItemEnum, LitInt, parse_macro_input};

#[derive(Copy, Clone, Debug, FromMeta)]
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
// impl FromStr for IntRepr {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "u8" => Ok(Self::U8),
//             "u16" => Ok(Self::U16),
//             "u32" => Ok(Self::U32),
//             "u64" => Ok(Self::U64),
//             "u128" => Ok(Self::U128),
//             "usize" => Ok(Self::Usize),
//             "i8" => Ok(Self::I8),
//             "i16" => Ok(Self::I16),
//             "i32" => Ok(Self::I32),
//             "i64" => Ok(Self::I64),
//             "i128" => Ok(Self::I128),
//             "isize" => Ok(Self::Isize),
//             _ => Err(()),
//         }
//     }
// }

#[derive(Copy, Clone, Debug, FromMeta)]
enum Parity {
    Even,
    Odd,
}

#[derive(Debug, Clone, FromMeta)]
#[darling(derive_syn_parse)]
struct BitParityArgs {
    parity: Parity,
    repr: IntRepr,
}

fn try_expand(args: BitParityArgs, mut enum_item: ItemEnum) -> syn::Result<TokenStream> {
    use quote::quote;
    let mut bpi = BitParityIter::new(matches!(args.parity, Parity::Even));

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
            let tok = syn::token::Eq;

            variant.discriminant = Some((tok, syn::parse_quote!(#next_disc)));
        }
    }

    Ok(quote! {enum_item})
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
