pub fn build(field_names: &[String], field_types: &[String], macro_name: &str) -> String {
    field_names.iter().zip(field_types.iter())
        .filter_map(|(f, t)| {
            if t.trim() != "Padding" { return None; }
            Some(format!(
                // none
                "(@build $expr:expr, {f}: none(), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::none()), $($rest)*] }};
                // all with Size variants
                (@build $expr:expr, {f}: all($v:ident), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::all(kairos::Space::Scale(kairos::Size::$v))), $($rest)*] }};
                (@build $expr:expr, {f}: all(Custom($v:expr)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::all(kairos::Space::Custom($v))), $($rest)*] }};
                (@build $expr:expr, {f}: all(Fill), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::all(kairos::Space::Fill)), $($rest)*] }};
                (@build $expr:expr, {f}: all(Shrink), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::all(kairos::Space::Shrink)), $($rest)*] }};
                // horizontal
                (@build $expr:expr, {f}: horizontal(Symmetrical($v:ident)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::horizontal(kairos::AxisPadding::Symmetrical(kairos::Space::Scale(kairos::Size::$v)))), $($rest)*] }};
                (@build $expr:expr, {f}: horizontal(Symmetrical(Custom($v:expr))), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::horizontal(kairos::AxisPadding::Symmetrical(kairos::Space::Custom($v)))), $($rest)*] }};
                (@build $expr:expr, {f}: horizontal(Custom($a:expr, $b:expr)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::horizontal(kairos::AxisPadding::Custom($a, $b))), $($rest)*] }};
                // vertical
                (@build $expr:expr, {f}: vertical(Symmetrical($v:ident)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::vertical(kairos::AxisPadding::Symmetrical(kairos::Space::Scale(kairos::Size::$v)))), $($rest)*] }};
                (@build $expr:expr, {f}: vertical(Symmetrical(Custom($v:expr))), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::vertical(kairos::AxisPadding::Symmetrical(kairos::Space::Custom($v)))), $($rest)*] }};
                (@build $expr:expr, {f}: vertical(Custom($a:expr, $b:expr)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::vertical(kairos::AxisPadding::Custom($a, $b))), $($rest)*] }};
                // symmetric
                (@build $expr:expr, {f}: symmetric(Symmetrical($h:ident), Symmetrical($v:ident)), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::symmetric(kairos::AxisPadding::Symmetrical(kairos::Space::Scale(kairos::Size::$h)), kairos::AxisPadding::Symmetrical(kairos::Space::Scale(kairos::Size::$v)))), $($rest)*] }};
                (@build $expr:expr, {f}: symmetric(Symmetrical(Custom($h:expr)), Symmetrical(Custom($v:expr))), $($rest:tt)*) => {{ {macro_name}![@build $expr.{f}(kairos::Padding::symmetric(kairos::AxisPadding::Symmetrical(kairos::Space::Custom($h)), kairos::AxisPadding::Symmetrical(kairos::Space::Custom($v)))), $($rest)*] }};",
            ))
        })
        .collect::<Vec<_>>()
        .join("\n")
}