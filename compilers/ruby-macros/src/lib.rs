#![warn(missing_docs)]

//! Procedural macros for the Rusty Ruby ecosystem
//!
//! This crate provides various procedural macros to simplify development
//! within the Rusty Ruby ecosystem.

use proc_macro::TokenStream;
use quote::quote;
use syn;

/// A simple example procedural macro
///
/// This macro demonstrates the basic structure of a procedural macro in this crate.
#[proc_macro]
pub fn example_macro(_input: TokenStream) -> TokenStream {
    // Example implementation that returns a simple expression
    let expanded = quote! {
        println!("Hello from ruby-macros!");
    };

    TokenStream::from(expanded)
}

/// A derive macro example
///
/// This macro demonstrates how to create a derive macro in this crate.
#[proc_macro_derive(ExampleDerive)]
pub fn example_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the output, possibly using quasi-quotation
    let generated = quote! {
        impl ExampleDerive for #ast {
            fn example_method(&self) {
                println!("Example derive macro implemented!");
            }
        }
    };

    TokenStream::from(generated)
}

/// An attribute macro example
///
/// This macro demonstrates how to create an attribute macro in this crate.
#[proc_macro_attribute]
pub fn example_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // For this example, we just return the item unchanged
    // In a real macro, you would process the attribute and modify the item
    item
}

/// A derive macro for defining Ruby classes
///
/// This macro generates code that defines a Ruby class with the same name as the struct,
/// handles struct fields appropriately, and integrates with the existing Ruby API.
#[proc_macro_derive(RubyClass)]
pub fn ruby_class_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Generate the Ruby class implementation
    impl_ruby_class(&ast)
}

/// Generate Ruby class implementation for a struct
fn impl_ruby_class(ast: &syn::DeriveInput) -> TokenStream {
    // Extract the struct name
    let struct_name = &ast.ident;
    let class_name = struct_name.to_string();

    // Extract struct fields
    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(ref fields), .. }) = ast.data {
        fields
    }
    else {
        panic!("RubyClass can only be derived for structs with named fields");
    };

    // Generate field names and types
    let field_names: Vec<_> = fields
        .named
        .iter()
        .map(|f| {
            let name = &f.ident;
            let name_str = name.as_ref().unwrap().to_string();
            quote! { #name_str }
        })
        .collect();

    let field_ids: Vec<_> = fields
        .named
        .iter()
        .map(|f| {
            let name = &f.ident;
            quote! { #name }
        })
        .collect();

    // Generate the implementation
    let expanded = quote! {
        impl #struct_name {
            /// Convert the struct to a Ruby Value
            pub fn to_ruby_value(&self) -> ::ruby_types::RubyValue {
                use ::ruby_types::RubyValue;

                let mut fields = std::collections::HashMap::new();
                #(fields.insert(#field_names.to_string(), self.#field_ids.to_ruby_value());)*

                RubyValue::Object(#class_name.to_string(), fields)
            }

            /// Define this class in the Ruby runtime
            pub fn define_ruby_class(ruby: &mut ::ruby::Ruby) -> Result<(), Box<dyn std::error::Error>> {
                ruby.define_class(#class_name)?;
                Ok(())
            }
        }

        impl ::ruby::ToRubyValue for #struct_name {
            fn to_ruby_value(&self) -> ::ruby_types::RubyValue {
                self.to_ruby_value()
            }
        }
    };

    TokenStream::from(expanded)
}
