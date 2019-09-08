#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::{Diagnostic, Level, TokenStream};
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::Fields::{Named, Unnamed, Unit};
use syn::{Ident, FieldsNamed, FieldsUnnamed};

fn explode_struct(struct_name: &Ident, fields: &FieldsNamed) -> TokenStream {
    let fields_infos = fields.named.iter().map(|field| {
        (field.ident.as_ref().unwrap(), &field.ty)
    });

    let (idents, types) = fields_infos.fold((quote!(), quote!()), |(idents, types), (ident, ty)| {
        (quote!(#idents #ident, ), quote!(#types #ty, ))
    });

    quote!(
        impl Explode for #struct_name {
            type Tuple = (#types);

            fn explode(self) -> Self::Tuple {
                let #struct_name { #idents } = self;

                (#idents)
            }
        }

        impl Into<(#types)> for #struct_name {
            fn into(self) -> (#types) {
                self.explode()
            }
        }
    ).into()
}

fn explode_tuple_struct(struct_name: &Ident, fields: &FieldsUnnamed) -> TokenStream {
    let (idents, types) = fields
        .unnamed
        .iter()
        .enumerate()
        .fold((quote!(), quote!()), |(idents, types), (nth, field)| {
            let ident = format_ident!("f{}", nth);
            let ty = &field.ty;
            (quote!(#idents #ident, ), quote!(#types #ty, ))
        });

    quote!(
        impl Explode for #struct_name {
            type Tuple = (#types);

            fn explode(self) -> Self::Tuple {
                let #struct_name ( #idents ) = self;

                (#idents)
            }
        }

        impl Into<(#types)> for #struct_name {
            fn into(self) -> (#types) {
                self.explode()
            }
        }
    ).into()
}

fn explode_impl(ast: &syn::DeriveInput) -> TokenStream {
    if let Struct(struct_data) = &ast.data {
        let struct_name = &ast.ident;

        match &struct_data.fields {
            Named(fields) => explode_struct(struct_name, fields),
            Unnamed(fields) => explode_tuple_struct(struct_name, fields),
            Unit => {
                Diagnostic::new(Level::Error, "'Explode' can only be derived for non-unit structs").emit();
                TokenStream::new()
            }
        }
    } else {
        Diagnostic::new(Level::Error, "'Explode' can only be derived for structs").emit();
        TokenStream::new()
    }
}

#[proc_macro_derive(Explode)]
pub fn explode(input: TokenStream) -> TokenStream {
    explode_impl(&syn::parse(input).unwrap())
}
