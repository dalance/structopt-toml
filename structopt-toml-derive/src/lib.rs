extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::parse::{ParseStream, Parse};
use syn::{DataStruct, DeriveInput, Field, Ident, LitStr, buffer::Cursor};


#[proc_macro_derive(StructOptToml, attributes(structopt))]
pub fn structopt_toml(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_structopt_toml(&input);
    gen.into()
}

fn impl_structopt_toml(input: &DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;

    let struct_name = &input.ident;
    let inner_impl = match input.data {
        Struct(DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => impl_structopt_for_struct(struct_name, &fields.named),
        _ => panic!("structopt_toml only supports non-tuple struct"),
    };

    quote!(#inner_impl)
}

fn impl_structopt_for_struct(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let merged_fields = gen_merged_fields(fields);

    quote! {
        impl ::structopt_toml::StructOptToml for #name {
            fn merge<'a>(from_toml: Self, from_args: Self, args: &::structopt_toml::clap::ArgMatches) -> Self where
                Self: Sized,
                Self: ::structopt_toml::structopt::StructOpt,
                Self: ::structopt_toml::serde::de::Deserialize<'a>
            {
                Self {
                    #merged_fields
                }
            }
        }

        impl Default for #name {
            fn default() -> Self {
                let args = vec!["bin"];
                #name::from_iter(args.iter())
            }
        }
    }
}

fn gen_merged_fields(fields: &Punctuated<Field, Comma>) -> proc_macro2::TokenStream {
    let fields = fields.iter().map(|field| {
        let explicit_name = field
            .attrs
            .iter()
            .filter(|&attr| attr.path.is_ident("structopt"))
            .filter_map(|attr| {
                // extract parentheses 
                let ts = attr.parse_args().ok()?;
                // find name = `value` in attribute
                syn::parse2::<NameVal>(ts).map(|nv| nv.0).ok()
            })
            .nth(0);

        // by default the clap arg name is the field name, unless overwritten with `name=<value>`
        let field_name = field.ident.as_ref().unwrap();
        let name_str = explicit_name.unwrap_or(format!("{}", field_name));
        let structopt_name = LitStr::new(&name_str, field_name.span());
        quote!(
            #field_name: {
                if args.is_present(#structopt_name) && args.occurrences_of(#structopt_name) > 0 {
                    from_args.#field_name
                } else {
                    from_toml.#field_name
                }
            }
        )
    });
    quote! (
        #( #fields ),*
    )
}

#[derive(Debug)]
struct NameVal(String);

impl Parse for NameVal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        #[derive(PartialEq, Eq, Debug)]
        enum Match {
            NameToken,
            PunctEq,
            LitVal,
        }
        let mut state = Match::NameToken;
        let result = input.step(|cursor| {
            let mut rest = *cursor;
            while let Some((tt, next)) = rest.token_tree() {
                match tt {
                    TokenTree::Ident(ident) if ident == "name" && state == Match::NameToken => {
                        state = Match::PunctEq;
                    }
                    TokenTree::Punct(punct) if punct.as_char() == '=' && state == Match::PunctEq => {
                        state = Match::LitVal;
                    }
                    TokenTree::Literal(lit) if state == Match::LitVal => {
                        return Ok((lit.to_string().replace("\"", ""), Cursor::empty()));
                    }
                    _ => {
                        // on first incorrect token reset
                        state = Match::NameToken;
                    }
                }
                rest = next;
            }
            Err(cursor.error("End reached"))
        });
        result.map(|lit| Self(lit)).map_err(|_| input.error("Not found"))
    }
}
