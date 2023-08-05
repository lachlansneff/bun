use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitInt, Token};

struct Ratio {
    num: i128,
    den: Option<u128>,
}

impl Parse for Ratio {
    fn parse(input: ParseStream) -> Result<Self> {
        let num = input.parse::<LitInt>()?.base10_parse::<i128>()?;
        let den = if input.parse::<Option<Token![/]>>()?.is_some() {
            Some(input.parse::<LitInt>()?.base10_parse::<u128>()?)
        } else {
            None
        };

        Ok(Self { num, den })
    }
}

fn gen_uint(n: u128) -> proc_macro2::TokenStream {
    if n == 0 {
        quote! { ::typenum::UTerm }
    } else {
        let ty = gen_uint(n / 2);
        let b = if n % 2 == 0 {
            quote! { ::typenum::B0 }
        } else {
            quote! { ::typenum::B1 }
        };
        quote! { ::typenum::UInt<#ty, #b> }
    }
}

#[proc_macro]
pub fn ratio(item: TokenStream) -> TokenStream {
    let ratio = parse_macro_input!(item as Ratio);

    let num = gen_uint(ratio.num.abs() as u128);
    let num = if ratio.num < 0 {
        quote! { ::typenum::NInt<#num>  }
    } else if ratio.num == 0 {
        quote! { ::typenum::Z0 }
    } else {
        quote! { ::typenum::PInt<#num> }
    };

    if let Some(den) = ratio.den {
        let den = gen_uint(den);
        quote! {
            ::bun::Ratio<#num, #den>
        }
    } else {
        quote! {
            ::bun::Ratio<#num>
        }
    }
    .into()
}
