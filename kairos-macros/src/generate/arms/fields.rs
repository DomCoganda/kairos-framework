

pub fn build(field_names: &[String], field_types: &[String], macro_name: &str) -> String {
    // One builder rule per field
    let field_rules: String = field_names.iter().zip(field_types.iter())
        .map(|(f, t)| {
            let val = if t.contains("Signal") {
                "$val.clone()".to_string()
            } else if t.contains("Option") && t.contains("Arc") && t.contains("dyn FnMut") {
                "Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || { $val }))))".to_string()
            } else if t.contains("Arc") && t.contains("dyn FnMut") {
                "std::sync::Arc::new(std::sync::Mutex::new(Box::new(move || { $val })))".to_string()
            } else if t.contains("Arc") && t.contains("dyn Fn") {
                "std::sync::Arc::new(move |_| $val)".to_string()
            } else if t.contains("dyn FnMut") {
                "Some(Box::new($val))".to_string()
            } else {
                "$val".to_string()
            };
            format!(
                "(@build $expr:expr, {f}: $val:expr, $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}({val}), $($rest)*] }};",
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    field_rules
}