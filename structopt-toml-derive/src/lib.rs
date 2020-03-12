extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DataStruct, DeriveInput, Field, Ident, Lit, LitStr, Meta, MetaNameValue, NestedMeta};
use syn::punctuated::Punctuated;
use syn::token::Comma;

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
        Struct(DataStruct { fields: syn::Fields::Named(ref fields), .. }) =>
            impl_structopt_for_struct(struct_name, &fields.named),
        _ => panic!("structopt_toml only supports non-tuple struct")
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
    use Meta::*;
    use NestedMeta::*;
    use Lit::*;

    let fields = fields.iter().map(|field| {
        let iter = field.attrs.iter()
            .filter_map(|attr| {
                if attr.path.is_ident("structopt") {
                    let meta = attr
                        .parse_meta()
                        .expect(&format!("invalid structopt syntax: {}", quote!(attr)));
                    Some(meta)
                } else {
                    None
                }
            }).
        flat_map(|m| match m {
            List(l) => l.nested,
            tokens => panic!("unsupported syntax: {}", quote!(#tokens).to_string()),
        })
        .map(|m| match m {
            Meta(m) => m,
            ref tokens => panic!("unsupported syntax: {}", quote!(#tokens).to_string()),
        });

        let mut structopt_name = LitStr::new(&format!("{}", field.ident.as_ref().unwrap().clone()), field.ident.as_ref().unwrap().span());
        for attr in iter {
            match attr {
                NameValue(MetaNameValue { path, lit: Str(value), .. }) => {
                    if path.is_ident("name") {
                        structopt_name = value;
                    }
                }
                _ => ()
            }
        }
        let field_name = field.ident.as_ref().unwrap();
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
