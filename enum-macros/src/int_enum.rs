use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemEnum};

pub fn int_enum(enum_: ItemEnum, type_: Ident) -> TokenStream {
    let enum_impl = impl_from_enum(&enum_, &type_);
    let type_impl = impl_try_from_type(&enum_, &type_);

    quote!{
        #enum_
        #enum_impl
        #type_impl
    }
}

fn impl_from_enum(e: &ItemEnum, t: &Ident) -> TokenStream {
    let ident = &e.ident;

    let cases = e.variants.iter().map(|v| {
        let v_ident = &v.ident;
        let v_discriminant = &v.discriminant.as_ref().unwrap().1;

        quote!{
            #ident::#v_ident => #v_discriminant
        }
    });

    quote! {
        impl std::convert::From<#ident> for #t {
            fn from(x: #ident) -> Self {
                match x {
                    #(#cases),*
                }
            }
        }
    }
}

fn impl_try_from_type(e: &ItemEnum, t: &Ident) -> TokenStream {
    let ident = &e.ident;

    let cases = e.variants.iter().map(|v| {
        let v_ident = &v.ident;
        let v_discriminant = &v.discriminant.as_ref().unwrap().1;

        quote!{
            #v_discriminant => std::result::Result::Ok(#ident::#v_ident)
        }
    });

    quote! {
        impl std::convert::TryFrom<#t> for #ident {
            type Error = #t;

            fn try_from(x: #t) -> std::result::Result<Self, #t> {
                match x {
                    #(#cases),*,
                    _ => std::result::Result::Err(x)
                }
            }
        }
    }
}
