use syn::Fields;

pub fn build(fields: &Fields, macro_name: &str) -> String {
    let push_arms: String = fields.iter()
        .filter_map(|f| {
            let has_push = f.attrs.iter().any(|a| a.path().is_ident("push"));
            if !has_push { return None; }
            let field_name = f.ident.as_ref()?.to_string();
            let push_method = format!("push_{field_name}");
            Some(format!(
                "(@build $expr:expr, {field_name}![$($inner:tt)*], $($rest:tt)*) => {{ {macro_name}![@build $expr.{push_method}({field_name}![$($inner)*]), $($rest)*] }};",
            ))
        })
        .collect::<Vec<_>>()
        .join("\n");
    push_arms
}