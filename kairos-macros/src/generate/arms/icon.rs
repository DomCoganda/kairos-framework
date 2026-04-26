

pub fn build(field_names: &[String], field_types: &[String], macro_name: &str) -> String {
    let icon_arms: String = field_names.iter().zip(field_types.iter())
        .filter_map(|(f, t)| {
            if t.trim() == "Icon" {
                Some(format!(
                    "(@build $expr:expr, {f}: ($path:literal), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(Icon {{ source: IconSource::Embedded(include_str!($path)), ..Icon::new() }}), $($rest)*] }};",
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    icon_arms
}