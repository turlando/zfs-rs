use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, TypePath};

pub fn int_enum(r#enum: ItemEnum, r#type: TypePath) -> TokenStream {
    let enum_impl = impl_from_enum::r#impl(&r#enum, &r#type);
    let type_impl = impl_try_from_type::r#impl(&r#enum, &r#type);

    quote!{
        #r#enum
        #enum_impl
        #type_impl
    }
}

pub mod impl_from_enum {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Ident, ItemEnum, TypePath, Variant};

    pub fn r#impl(e: &ItemEnum, dest_type: &TypePath) -> TokenStream {
        let enum_name = &e.ident;
        let from = from(e);

        quote! {
            impl ::core::convert::From<#enum_name> for #dest_type {
                #from
            }
        }
    }

    fn from(
        ItemEnum{ ident: enum_name, variants, .. }: &ItemEnum
    ) -> TokenStream {
        let cases = variants.iter().map(|v| case(enum_name, v));

        quote! {
            fn from(x: #enum_name) -> Self {
                match x {
                    #(#cases),*
                }
            }
        }
    }

    fn case(
        enum_name: &Ident,
        Variant { ident, discriminant, .. }: &Variant
    ) -> TokenStream {
        // FIXME: unwrap()
        let d = &discriminant.as_ref().unwrap().1;

        quote!{
            #enum_name::#ident => #d
        }
    }
}

pub mod impl_try_from_type {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Ident, ItemEnum, TypePath, Variant};

    pub fn r#impl(e: &ItemEnum, dest_type: &TypePath) -> TokenStream {
        let enum_name = &e.ident;
        let try_from = try_from(e, dest_type);

        quote! {
            impl ::core::convert::TryFrom<#dest_type> for #enum_name {
                type Error = #dest_type;
                #try_from
            }
        }
    }

    fn try_from(
        ItemEnum{ ident: enum_name, variants, .. }: &ItemEnum,
        dest_type: &TypePath
    ) -> TokenStream {
        let cases = variants.iter().map(|v| case(enum_name, v));

        quote! {
            fn try_from(
                x: #dest_type
            ) -> ::core::result::Result<Self, #dest_type> {
                match x {
                    #(#cases),*,
                    _ => ::core::result::Result::Err(x)
                }
            }
        }
    }

    fn case(
        enum_name: &Ident,
        Variant { ident, discriminant, .. }: &Variant
    ) -> TokenStream {
        // FIXME: unwrap()
        let d = &discriminant.as_ref().unwrap().1;

        quote!{
            #d => ::core::result::Result::Ok(#enum_name::#ident)
        }
    }
}
