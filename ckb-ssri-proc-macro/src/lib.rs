extern crate proc_macro;

use core::panic;

use ckb_hash::blake2b_256;
use ckb_ssri::prelude::encode_u64_vector;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, ExprLit, Ident, Lit, Token};

struct Methods {
    argv: Expr,
    invalid_method: Expr,
    invalid_args: Expr,
    method_keys: Vec<u64>,
    method_bodies: Vec<Expr>,
}

fn method_path(name: impl AsRef<[u8]>) -> u64 {
    u64::from_le_bytes(blake2b_256(name)[0..8].try_into().unwrap())
}

impl Parse for Methods {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let argv = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let invalid_method = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let invalid_args = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;

        let mut method_keys = vec![];
        let mut method_bodies = vec![];
        while !input.is_empty() {
            let name = match input.parse::<Expr>()? {
                Expr::Lit(ExprLit {
                    lit: Lit::Str(v), ..
                }) => v.value(),
                _ => panic!("method name should be a string"),
            };
            input.parse::<Token![=>]>()?;
            let body = input.parse::<Expr>()?;
            input.parse::<Token![,]>()?;

            method_keys.push(method_path(name));
            method_bodies.push(body);
        }

        Ok(Methods {
            argv,
            invalid_method,
            invalid_args,
            method_keys,
            method_bodies,
        })
    }
}

#[proc_macro]
pub fn ssri_methods(input: TokenStream) -> TokenStream {
    let Methods {
        argv,
        invalid_method,
        invalid_args,
        method_keys,
        method_bodies,
    } = parse_macro_input!(input as Methods);

    let version_path = method_path("SSRI.version");
    let get_methods_path = method_path("SSRI.get_methods");
    let has_methods_path = method_path("SSRI.has_methods");

    let raw_methods = encode_u64_vector(
        [version_path, get_methods_path, has_methods_path]
            .iter()
            .chain(method_keys.iter())
            .copied()
            .collect::<Vec<_>>(),
    );
    let raw_methods_len = raw_methods.len();

    TokenStream::from(quote! {
        {
            use alloc::{borrow::Cow, vec::Vec};
            use ckb_std::high_level::decode_hex;
            const raw_methods: [u8; #raw_methods_len] = [#(#raw_methods,)*];
            let res: Result<Cow<'static, [u8]>, Error> = match u64::from_le_bytes(
                decode_hex(&(#argv)[0])?.try_into().map_err(|_| #invalid_method)?,
            ) {
                #version_path => Ok(Cow::from(&[0][..])),
                #get_methods_path => {
                    let offset = usize::min((4 +u64::from_le_bytes(
                        decode_hex(&(#argv)[1])?
                            .try_into()
                            .map_err(|_| #invalid_args)?
                    ) as usize * 8), #raw_methods_len);
                    let limit = usize::min((4 + (offset + u64::from_le_bytes(
                        decode_hex(&(#argv)[2])?
                            .try_into()
                            .map_err(|_| #invalid_args)?
                    ) as usize) * 8), #raw_methods_len);
                    if limit == 0 {
                        Ok(Cow::from(&raw_methods[offset..]))
                    } else {
                        Ok(Cow::from(&raw_methods[offset..limit]))
                    }
                },
                #has_methods_path => Ok(Cow::from(
                    decode_hex(&(#argv)[1])?[4..].chunks(8).map(|path| {
                        match raw_methods[4..]
                            .chunks(8)
                            .find(|v| v == &path) {
                                Some(_) => 1,
                                None => 0,
                            }
                    }).collect::<Vec<_>>()
                )),
                #(
                    #method_keys => #method_bodies,
                )*
                _ => Err(#invalid_method),
            };
            res
        }
    })
}
