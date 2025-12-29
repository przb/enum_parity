use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum, Meta};

fn try_expand(args: Meta, mut enum_item: ItemEnum) -> syn::Result<TokenStream> {
    let start = match args {
        Meta::Path(path) if path.is_ident("even") => 0,
        Meta::Path(path) if path.is_ident("odd") => 1,
        p => {
            return Err(syn::Error::new_spanned(
                p.path(),
                "expected `even` or `odd`",
            ))
        }
    };

    let mut value = start;

    for variant in &mut enum_item.variants {
        if variant.discriminant.is_some() {
            return Err(syn::Error::new_spanned(
                &variant.ident,
                "explicit discriminants are not allowed",
            ));
        }

        variant.discriminant = Some((syn::token::Eq::default(), syn::parse_quote!(#value)));

        value += 2;
    }

    Ok(quote! {
        #enum_item
    }
    .into())
}

#[proc_macro_attribute]
pub fn parity_enum(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as Meta);
    let enum_item = parse_macro_input!(input as ItemEnum);

    try_expand(args, enum_item)
        .map(Into::into)
        .unwrap_or_else(|e| e.into_compile_error().into())
}
