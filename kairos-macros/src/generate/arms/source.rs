pub fn build(field_names: &[String], field_types: &[String], macro_name: &str) -> String {
    let source_arms: String = field_names.iter().zip(field_types.iter())
        .filter_map(|(f, t)| {
            if t.trim() == "IconSource" {
                Some(format!(
                    "(@build $expr:expr, {f}: Path(File($path:literal)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::IconSource::Path(kairos::FileSource::File(include_str!($path).to_string()))), $($rest)*] }};
                    (@build $expr:expr, {f}: Path(File($path:expr)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::IconSource::Path(kairos::FileSource::File($path.to_string()))), $($rest)*] }};
                    (@build $expr:expr, {f}: Path(Embedded($path:literal)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::IconSource::Path(kairos::FileSource::Embedded(include_str!($path).to_string()))), $($rest)*] }};
                    (@build $expr:expr, {f}: Path(Url($path:literal)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::IconSource::Path(kairos::FileSource::Url($path))), $($rest)*] }};
                    (@build $expr:expr, {f}: System($path:literal), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::IconSource::System($path.to_string())), $($rest)*] }};
                    (@build $expr:expr, {f}: Raw($val:literal), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::IconSource::Raw($val.to_string())), $($rest)*] }};",
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    source_arms
}