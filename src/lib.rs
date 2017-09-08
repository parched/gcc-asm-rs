//   Copyright 2017 James Duley
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::str::FromStr;
use syn::{parse_token_trees, Token, TokenTree, Lit, Delimited, DelimToken, StrStyle, Mac, Path};
use quote::{Tokens, ToTokens};

#[proc_macro]
pub fn gcc_asm(token_stream: TokenStream) -> TokenStream {
    let tokens = token_stream.to_string();
    let token_trees = parse_token_trees(&tokens).unwrap();
    let mut parts = split_on_token(token_trees.as_slice(), &Token::Colon);

    let template = parts
        .next()
        .expect("error: template missing")
        .iter()
        .map(get_string_literal)
        // support C-style string literal concatenation
        .fold(String::new(), |acc, ref x| acc + &*x);
    let output_operands = parts.next().unwrap_or(&[]);
    let input_operands = parts.next().unwrap_or(&[]);
    let clobbers = parts.next().unwrap_or(&[]);

    assert!(
        clobbers.len() == 0usize,
        "error: clobbers not supported yet"
    );
    assert!(parts.next().is_none(), "error: extra tokens after clobbers");

    let mut symbolic_names = Vec::new();
    let new_output_operands = split_on_token(output_operands, &Token::Comma)
        .map(|tts| extract_symbolic_name(&mut symbolic_names, tts))
        .collect::<Vec<_>>();
    let new_input_operands = split_on_token(input_operands, &Token::Comma)
        .map(|tts| extract_symbolic_name(&mut symbolic_names, tts))
        .collect::<Vec<_>>();
    let new_template = replace_template(template, symbolic_names.as_slice());

    let mut new_tokens =
        quote! {
        asm!(#new_template : #(#(#new_output_operands)*),* : #(#(#new_input_operands)*),*)
    };
    // println!("{}", new_tokens);
    TokenStream::from_str(new_tokens.as_str()).unwrap()
}

fn split_on_token<'a>(
    token_trees: &'a [TokenTree],
    separator: &'a Token,
) -> Box<Iterator<Item = &'a [TokenTree]> + 'a> {
    if token_trees.is_empty() {
        Box::new(std::iter::empty())
    } else {
        Box::new(token_trees.split(move |tt| match *tt {
            TokenTree::Token(ref token) => token == separator,
            _ => false,
        }))
    }
}

fn replace_template(template: String, symbolic_names: &[Option<String>]) -> String {
    let with_dollars = template.replace("$", "\u{80}").replace("%", "$").replace(
        "$$",
        "%",
    );

    symbolic_names
        .iter()
        .enumerate()
        .fold(with_dollars, |acc, ref number_and_name| {
            if let Some(ref x) = *number_and_name.1 {
                acc.replace(
                    &("$[".to_string() + &x + "]"),
                    &("$".to_string() + &number_and_name.0.to_string()),
                )
            } else {
                acc
            }
        })
        .replace("$=", "${:uid}")
        .replace("\u{80}", "$$")
}

fn extract_symbolic_name<'a>(
    ordered_list: &mut Vec<Option<String>>,
    tts: &'a [TokenTree],
) -> &'a [TokenTree] {
    let name_and_remaining = match *tts.first().expect("error: empty operand") {
        TokenTree::Delimited(ref d) => {
            assert!(d.delim == DelimToken::Bracket, "error: bad operand");
            let name = if d.tts.len() == 1usize {
                match d.tts[0] {
                    TokenTree::Token(Token::Ident(ref name)) => Some(name.to_string()),
                    _ => None,
                }
            } else {
                None
            };
            assert!(name.is_some(), "error: bad symbolic name");
            (name, tts.split_at(1).1)
        }
        _ => (None, tts),
    };
    ordered_list.push(name_and_remaining.0);
    name_and_remaining.1
}

fn get_string_literal(tt: &TokenTree) -> &String {
    match *tt {
        TokenTree::Token(Token::Literal(Lit::Str(ref string, _))) => string,
        _ => panic!("error: expected a string literal"),
    }
}
