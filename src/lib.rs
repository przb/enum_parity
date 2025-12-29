mod bit_par_iter;

use bit_par_iter::BitParityIter;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, Meta, parse_macro_input};

fn try_expand(args: Meta, mut enum_item: ItemEnum) -> syn::Result<TokenStream> {
    let is_even = match args {
        Meta::Path(path) if path.is_ident("even") => true,
        Meta::Path(path) if path.is_ident("odd") => false,
        p => {
            return Err(syn::Error::new_spanned(
                p.path(),
                "expected `even` or `odd`",
            ));
        }
    };

    let mut value_iter = BitParityIter::<u64>::new(is_even);

    for variant in &mut enum_item.variants {
        let value = value_iter
            .next()
            .ok_or_else(|| syn::Error::new_spanned(&variant, "ran out of discriminants"))?;
        if variant.discriminant.is_some() {
            return Err(syn::Error::new_spanned(
                &variant.ident,
                "explicit discriminants are not allowed",
            ));
        }

        variant.discriminant = Some((syn::token::Eq::default(), syn::parse_quote!(#value)));
    }

    Ok(quote! {
        #enum_item
    }
    .into())
}

#[proc_macro_attribute]
pub fn bit_parity(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as Meta);
    let enum_item = parse_macro_input!(input as ItemEnum);

    try_expand(args, enum_item)
        .map(Into::into)
        .unwrap_or_else(|e| e.into_compile_error().into())
}
