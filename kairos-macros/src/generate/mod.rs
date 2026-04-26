mod arms;
mod impl_methods;
mod output;

use heck::ToSnakeCase;
use quote::{quote, ToTokens};
use syn::DeriveInput;

pub fn generate(input: &DeriveInput, wrap_widget: bool) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let macro_name = name.to_string().to_snake_case();
    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        panic!("can only be used on structs")
    };

    let mut clean_input = input.clone();
    clean_input.attrs.retain(|attr| !attr.path().is_ident("export_macro"));

    let has_clone = clean_input.attrs.iter().any(|a| {
        a.to_token_stream().to_string().contains("Clone")
    });
    if !has_clone {
        let clone_attr: syn::Attribute = syn::parse_quote!(#[derive(Clone)]);
        clean_input.attrs.push(clone_attr);
    }
    if let syn::Data::Struct(ref mut data) = clean_input.data {
        for field in data.fields.iter_mut() {
            field.attrs.retain(|attr| !attr.path().is_ident("nested") && !attr.path().is_ident("push") && !attr.path().is_ident("widget"));
        }
    }

    let field_names: Vec<String> = fields.iter()
        .filter_map(|f| f.ident.as_ref())
        .map(|f| f.to_string())
        .collect();

    let field_types: Vec<String> = fields.iter()
        .filter_map(|f| f.ident.as_ref().map(|_| {
            let ty = &f.ty;
            quote!(#ty).to_string()
        }))
        .collect();

    let nested_arms = arms::nested::build(fields, &macro_name);
    let icon_arms = arms::icon::build(&field_names, &field_types, &macro_name);
    let label_arms = arms::label::build(&field_names, &field_types, &macro_name);
    let source_arms = arms::source::build(&field_names, &field_types, &macro_name);
    let push_arms = arms::push::build(fields, &macro_name);
    let padding_arms = arms::padding::build(&field_names, &field_types, &macro_name);
    let field_rules = arms::fields::build(&field_names, &field_types, &macro_name);
    let registry_arms = arms::registry::build(&field_names, &field_types, &macro_name);
    let impl_methods_str = impl_methods::build(&field_names, &field_types, fields, &macro_name);

    output::build(
        input,
        &clean_input,
        name,
        &macro_name,
        wrap_widget,
        &nested_arms,
        &icon_arms,
        &label_arms,
        &source_arms,
        &push_arms,
        &padding_arms,
        &field_rules,
        &registry_arms,
        &impl_methods_str,
    )

}
