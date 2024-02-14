use enum_macros_common::{bitmask::bitmask, int_enum::impl_try_from_type};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemEnum, TypePath, parse2};
use std::str::FromStr;

// This is already defined in ::xdr::primitive, but redefining it here allows us
// to avoid a circular dependency between the xdr and the xdr-macros crates.
const I32_SIZE: usize = 4;

pub fn derive_enum(e: &ItemEnum) -> TokenStream {
    let impl_try_from_type = impl_try_from_type::r#impl(&e, &i32());
    let r#impl = r#impl(&e.ident);

    quote!{
        #impl_try_from_type
        #r#impl
    }
}

pub fn derive_bitmask(e: &ItemEnum) -> TokenStream {
    let derive_enum = derive_enum(&e);
    let bitmask = bitmask(&e, &i32());

    quote!{
        #derive_enum
        #bitmask
    }
}

fn i32() -> TypePath {parse2::<TypePath>(
    // The following unwrap()s can't fail (I hope).
    TokenStream::from_str("::core::primitive::i32").unwrap()).unwrap()
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
