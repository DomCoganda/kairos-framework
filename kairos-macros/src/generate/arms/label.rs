pub fn build(field_names: &[String], field_types: &[String], macro_name: &str) -> String {
    let label_arms: String = field_names.iter().zip(field_types.iter())
        .filter_map(|(f, t)| {
            if t.trim() == "LabelContent" {
                Some(format!(
                    "(@build $expr:expr, {f}: icon![$($icon:tt)*], $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::LabelContent::Icon(Box::new(kairos::icon![$($icon)*]))), $($rest)*] }};\n\
                    (@build $expr:expr, {f}: Both($text:expr, icon![$($icon:tt)*]), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::LabelContent::Both(kairos::Text::from($text), Box::new(kairos::icon![$($icon)*]))), $($rest)*] }};",
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    label_arms
}