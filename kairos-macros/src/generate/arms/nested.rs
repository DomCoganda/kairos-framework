use heck::ToSnakeCase;
use quote::quote;
use syn::Fields;

pub fn build(fields: &Fields, macro_name: &str) -> String {
    let nested_arms: String = fields.iter()
        .filter_map(|f| {
            let has_nested = f.attrs.iter().any(|a| a.path().is_ident("nested"));
            if !has_nested { return None; }

            let field_name = f.ident.as_ref()?.to_string();
            let ty = &f.ty;
            let ty_str = quote!(#ty).to_string();
            let type_name = if ty_str.starts_with("Option <") {
                ty_str.trim_start_matches("Option <")
                    .trim_end_matches('>')
                    .trim()
                    .to_string()
            } else {
                ty_str.clone()
            };
            let nested_macro_trigger = field_name.clone();
            let nested_macro_expand = type_name.to_snake_case();
            let is_option = ty_str.starts_with("Option");
            let type_pascal = type_name.clone();

            let call = if is_option {
                format!("$expr.{field_name}(Some({nested_macro_expand}![$($inner)*]))")
            } else {
                format!("$expr.{field_name}({nested_macro_expand}![$($inner)*])")
            };
            Some(format!(
                "(@build $expr:expr, {nested_macro_trigger}![$($inner:tt)*], $($rest:tt)*) => {{ {macro_name}![@build {call}, $($rest)*] }};
                (@build $expr:expr, {type_pascal}![$($inner:tt)*], $($rest:tt)*) => {{ {macro_name}![@build {call}, $($rest)*] }};",
            ))
        })
        .collect::<Vec<_>>()
        .join("\n");
    nested_arms
}