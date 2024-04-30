/*
 * Copyright (C) 2024 Polkadot Blockchain Academy
 *  See the LICENSE.md file distributed with this work for additional
 *  information regarding copyright ownership.
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *      http://www.apache.org/licenses/LICENSE-2.0
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Named)]
pub fn named_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl Named for #name {
            fn name(&self) -> &str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}
