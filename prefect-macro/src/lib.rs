#![feature(unboxed_closures, fn_traits)]
extern crate proc_macro;
use std::mem;
use syn::*;
extern crate syn;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use std::collections::HashSet as Set;

use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, Expr, ExprClosure, FnArg, ItemFn, Pat, Path, Token};

struct Args {
    vars: Set<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

// impl<F> Fn<(String)> for Task<F>
// where
//     F: FnMut(),
// {
//     extern "rust-call" fn call(&self, _args: ()) -> String {
//         (self.callable)(_args)
//     }
// }

fn transform_params(params: Punctuated<FnArg, Token![,]>) -> Punctuated<Ident, Token![,]> {
    // 1. Filter the params, so that only typed arguments remain
    // 2. Extract the ident (in case the pattern type is ident)
    let idents = params.iter().filter_map(|param| {
        if let syn::FnArg::Typed(pat_type) = param {
            if let syn::Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                return Some(pat_ident.ident);
            }
        }
        None
    });

    // Add all idents to a Punctuated => param1, param2, ...
    let mut punctuated: Punctuated<syn::Ident, Token![,]> = Punctuated::new();
    idents.for_each(|ident| punctuated.push(ident));

    punctuated
}

#[proc_macro_attribute]
pub fn task(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let mut macro_args = parse_macro_input!(args as Args);

    let id = Ident::new(
        &("_".to_owned() + &input.sig.ident.to_string()),
        Span::call_site(),
    );
    let inner_ident = mem::replace(&mut input.sig.ident, id);

    let mut i = 0;
    let mut closure_inputs: Punctuated<Pat, Token![,]> = Punctuated::new();
    let mut arg_tuple = transform_params(input.sig.inputs.clone());
    let mut type_tuple: Punctuated<Type, Token![,]> = Punctuated::new();

    for arg in input.sig.inputs.iter_mut() {
        match arg {
            FnArg::Typed(pat) => {
                type_tuple.push(*pat.ty.clone());
                type_tuple.push_punct(Token![,]([Span::call_site()]));
                closure_inputs.push(Pat::from(pat.clone()));
            }

            _ => panic!("Unexpected argument {:?}", arg),
        }
    }

    let input_block = &input.block;
    let name = &input.sig.ident;
    let attributes = &input.attrs;
    let vis = &input.vis;
    let constness = &input.sig.constness;
    let unsafety = &input.sig.unsafety;
    let abi = &input.sig.abi;
    let generics = &input.sig.generics;
    let output = &input.sig.output;

    TokenStream::from(quote! {


        let mut #inner_ident = |#closure_inputs| {

           return tokio::task::spawn(async #input_block)

        };



    })
}

#[proc_macro_attribute]
pub fn flow(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let mut macro_args = parse_macro_input!(args as Args);

    let id = Ident::new(
        &("_".to_owned() + &input.sig.ident.to_string()),
        Span::call_site(),
    );
    let inner_ident = mem::replace(&mut input.sig.ident, id);

    let mut closure_inputs = Punctuated::new();
    let mut arg_tuple = transform_params(input.sig.inputs.clone());
    let mut type_tuple: Punctuated<Type, Token![,]> = Punctuated::new();

    for arg in input.sig.inputs.iter_mut() {
        match arg {
            FnArg::Typed(pat) => {
                type_tuple.push(*pat.ty.clone());
                type_tuple.push_punct(Token![,]([Span::call_site()]));
                closure_inputs.push(Pat::from(pat.clone()));
            }

            _ => panic!("Unexpected argument {:?}", arg),
        }
    }

    let closure = ExprClosure {
        attrs: input.attrs.clone(),
        asyncness: input.sig.asyncness,
        movability: None,
        capture: None,
        or1_token: Token![|]([Span::call_site()]),
        inputs: closure_inputs,
        or2_token: Token![|]([Span::call_site()]),
        output: input.sig.output.clone(),
        body: Box::new(Expr::Block(ExprBlock {
            label: None,
            attrs: vec![],
            block: *input.block.clone(),
        })),
    };
    let input_block = &input.block;

    let name = &input.sig.ident;
    let attributes = &input.attrs;
    let vis = &input.vis;
    let constness = &input.sig.constness;
    let unsafety = &input.sig.unsafety;
    let abi = &input.sig.abi;
    let generics = &input.sig.generics;
    let output = &input.sig.output;

    let excutor_name = &("_".to_owned() + &input.sig.ident.to_string());
    let flow_name = &input.sig.ident.to_string();
    TokenStream::from(quote! {
        let #inner_ident = || {

            let exec = DedicatedExecutor::new(#excutor_name, 2);

            let flow_context = exec.spawn(async move #input_block);
        };
    })
}

// impl<F> FnMut<(#type_tuple)> for Task<F>
// where
//     F: FnMut(#type_tuple) #output,
// {
//     extern "rust-call" fn call_mut(&mut self, _args: (#type_tuple)) -> i32 {
//         let (#arg_tuple) = _args;
//         (self.callable)(#arg_tuple)
//     }
// }

// impl<F> FnOnce<(#type_tuple)> for Task<F>
// where
//     F: FnMut(#type_tuple) #output,
// {
//     type Output = i32;

//     extern "rust-call" fn call_once(mut self, _args: (#type_tuple)) -> i32 {
//         let (#arg_tuple) = _args;
//         (self.callable)(#arg_tuple)
//     }
// }

// impl<F> FnMut<(#type_tuple)> for Flow<F>
//             where
//                 F: FnMut(#type_tuple) #output,
//             {
//                 extern "rust-call" fn call_mut(&mut self, _args: (#type_tuple)) -> i32 {
//                     let (#arg_tuple) = _args;
//                     (self.callable)(#arg_tuple)
//                 }
//             }

//             impl<F> FnOnce<(#type_tuple)> for Task<F>
//             where
//                 F: FnMut(#type_tuple) #output,
//             {
//                 type Output = i32;

//                 extern "rust-call" fn call_once(mut self, _args: (#type_tuple)) -> i32 {
//                     let (#arg_tuple) = _args;
//                     (self.callable)(#arg_tuple)
//                 }
//             }
