use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, TypePath};

pub fn int_enum(e: &ItemEnum, t: &TypePath) -> TokenStream {
    match common::get_variants(e) {
        Ok(vs) => {
            let enum_impl = impl_from_enum::r#impl(&e.ident, &vs, &t);
            let type_impl = impl_try_from_type::r#impl(&e.ident, &vs, &t);
            quote! { #e #enum_impl #type_impl }
        },
        Err(err) => {
            let err = err.to_compile_error();
            quote! { #e #err }
        }
    }
}

pub mod common {
    use syn::{Error, Expr, Ident, ItemEnum, Variant as SynVariant};

    pub type Variants<'a> = Vec<Variant<'a>>;

    pub struct Variant<'a> {
        pub ident: &'a Ident,
        pub expr: &'a Expr
    }

    pub fn get_variants(e: &ItemEnum) -> Result<Variants, Error> {
        let mut variants = Vec::with_capacity(e.variants.len());
        for SynVariant { ident, discriminant, .. } in e.variants.iter() {
            match discriminant.as_ref() {
                Some(d) => variants.push(Variant { ident: &ident, expr: &d.1 }),
                None => return Err(Error::new(
                    ident.span(),
                    "explicit discriminant value is required"
                ))
            }
        }
        Ok(variants)
    }
}

pub mod impl_from_enum {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Ident, TypePath};
    use crate::int_enum::common::{Variant, Variants};

    pub fn r#impl(
        enum_ident: &Ident,
        enum_variants: &Variants,
        dest_type: &TypePath
    ) -> TokenStream {
        let from = from(enum_ident);
        let from_ref = from_ref(enum_ident, enum_variants);
        quote! {
            impl ::core::convert::From<#enum_ident> for #dest_type { #from }
            impl ::core::convert::From<&#enum_ident> for #dest_type { #from_ref }
        }
    }

    fn from(enum_ident: &Ident) -> TokenStream {
        quote! { fn from(x: #enum_ident) -> Self { Self::from(&x) } }
    }

    fn from_ref(
        enum_ident: &Ident,
        enum_variants: &Variants,
    ) -> TokenStream {
        let cases = enum_variants.iter().map(
            |Variant { ident, expr }| quote! { #enum_ident::#ident => #expr }
        );
        quote! { fn from(x: &#enum_ident) -> Self { match x { #(#cases),* } } }
    }
}

pub mod impl_try_from_type {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Ident, TypePath};
    use crate::int_enum::common::{Variant, Variants};

    pub fn r#impl(
        enum_ident: &Ident,
        enum_variants: &Variants,
        dest_type: &TypePath
    ) -> TokenStream {
        let try_from = try_from(enum_ident, enum_variants, dest_type);
        quote! {
            impl ::core::convert::TryFrom<#dest_type> for #enum_ident {
                type Error = #dest_type;
                #try_from
            }
        }
    }

    fn try_from(
        enum_ident: &Ident,
        enum_variants: &Variants,
        dest_type: &TypePath
    ) -> TokenStream {
        let cases = enum_variants.iter().map(
            |Variant { ident, expr }|
            quote! { #expr => ::core::result::Result::Ok(#enum_ident::#ident) }
        );
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
}
