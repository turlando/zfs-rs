use crate::int_enum::impl_from_enum;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemEnum, TypePath, Variant};

pub fn bitmask(e: &ItemEnum, t: &TypePath) -> TokenStream {
    let r#impl = r#impl(&e, &t);
    let impl_from_enum = impl_from_enum::r#impl(&e, &t);

    quote!{
        #r#impl
        #impl_from_enum
    }
}

fn r#impl(e: &ItemEnum, t: &TypePath) -> TokenStream {
    let enum_name = &e.ident;
    let mapping_name = mapping_name(enum_name);
    let variant_count = e.variants.len();
    let variant_tuples = e.variants.iter().map(|v| variant_tuple(&e.ident, v));
    let values = values(e, t);

    quote!{
        const #mapping_name: [(#enum_name, #t); #variant_count] = [
            #(#variant_tuples),*
        ];
        impl #enum_name {
            #values
        }
    }
}

fn values(e: &ItemEnum, t: &TypePath) -> TokenStream {
    let mapping_name = mapping_name(&e.ident);

    quote!{
        // TODO: use ::alloc::vec instead of ::std::vec
        pub fn values(&self) -> ::std::vec::Vec<Self> {
            let v: #t = Into::<i32>::into(self);

            // TODO: Err if garbage in i.
            // TODO: Collect into a preallocated Vec with reasonable size.
            #mapping_name.iter()
                .filter(|(_, i)| *i & v != 0)
                .map(|(value, _)| *value)
                .collect()

        }
    }
}

fn variant_tuple(
    enum_name: &Ident,
    Variant { ident, discriminant, .. }: &Variant
) -> TokenStream {
    // FIXME: unwrap()
    let d = &discriminant.as_ref().unwrap().1;

    quote!{
        (#enum_name::#ident, #d)
    }
}

fn mapping_name(enum_name: &Ident) -> Ident {
    Ident::new(
        format!("__{}_MAPPING", enum_name.to_string().to_uppercase()).as_str(),
        enum_name.span() // FIXME: uhmmmmm
    )
}
