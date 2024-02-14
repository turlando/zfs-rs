use enum_macros_common::int_enum::impl_try_from_type;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemEnum};

// TODO: Remove as this is already defined in ::xdr::primitive.
const I32_SIZE: usize = 4;

pub fn derive_enum(r#enum: ItemEnum) -> TokenStream {
    let impl_try_from_type = impl_try_from_type::r#impl(
        &r#enum, 
        &Ident::new("i32", Span::call_site())
    );
    let r#impl = r#impl(&r#enum.ident);

    quote!{
        #impl_try_from_type
        #r#impl
    }
}

fn r#impl(enum_name: &Ident) -> TokenStream {
    let decode = decode();
    let read = read();

    quote!{
        impl #enum_name {
            #read
            #decode
        }
    }
}

fn read() -> TokenStream {
    quote!{
        pub fn read(
            r: &mut ::binary::Reader
        ) -> ::std::io::Result<Self> {
            r.try_read_as::<Self, std::io::Error, #I32_SIZE>(
                |x|
                match Self::decode(x) {
                    ::core::result::Result::Ok(v) => ::std::io::Result::Ok(v),
                    ::core::result::Result::Err(n) => ::std::io::Result::Err(
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            n.to_string()
                        )
                    )
                }
            )
        }
    }
}

fn decode() -> TokenStream {
    quote!{
        pub fn decode(
            x: &[::core::primitive::u8; #I32_SIZE]
        ) -> ::core::result::Result<Self, ::core::primitive::i32> {
            let v: ::core::primitive::i32 = ::xdr::I32::decode(x).into();
            Self::try_from(v)
        }
    }
}
