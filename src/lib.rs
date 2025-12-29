use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemEnum, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn parity_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut enum_item = parse_macro_input!(input as ItemEnum);

    // Parse parity argument
    let start = match args.as_slice() {
        [NestedMeta::Meta(Meta::Path(path))] if path.is_ident("even") => 0,
        [NestedMeta::Meta(Meta::Path(path))] if path.is_ident("odd") => 1,
        _ => {
            return syn::Error::new_spanned(quote! { #(#args),* }, "expected `even` or `odd`")
                .to_compile_error()
                .into();
        }
    };

    let mut value = start;

    for variant in &mut enum_item.variants {
        if variant.discriminant.is_some() {
            return syn::Error::new_spanned(
                &variant.ident,
                "explicit discriminants are not allowed",
            )
            .to_compile_error()
            .into();
        }

        variant.discriminant = Some((syn::token::Eq::default(), syn::parse_quote!(#value)));

        value += 2;
    }

    TokenStream::from(quote! {
        #enum_item
    })
}
