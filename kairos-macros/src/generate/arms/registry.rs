use crate::registry::{VariantKind, build_registry};

pub fn build(field_names: &[String], field_types: &[String], macro_name: &str) -> String {

    let registry = build_registry();
    let mut arms = String::new();

    for (field, type_str) in field_names.iter().zip(field_types.iter()) {

        let raw = type_str.trim();
        let inner = if let Some(start) = raw.find('<') {
            if let Some(end) = raw.rfind('>') {
                raw[start + 1..end].trim()
            } else { raw }
        } else { raw };
        let last_segment = inner.split("::").last().map(|s| s.trim()).unwrap_or(inner);
        let type_name = match last_segment {
            "Fill" | "BackgroundFill" => "PaletteFill",
            other => other,
        };

        let Some(typedef) = registry.iter().find(|t| {
            t.full_path.split("::").last().map(|s| s.trim()) == Some(type_name)
        }) else { continue };

        let path = &typedef.full_path;

        for variant in &typedef.variants {
            let vname = variant.name;
            let arm = match &variant.kind {
                VariantKind::Unit => format!(
                    "(@build $expr:expr, {field}: {vname}, $($rest:tt)*) => {{ {macro_name}![@build $expr.{field}({path}::{vname}), $($rest)*] }};\n"
                ),
                VariantKind::OpaqueData => {
                    let base = format!(
                        "(@build $expr:expr, {field}: {vname}($val:expr), $($rest:tt)*) => {{ {macro_name}![@build $expr.{field}({path}::{vname}($val)), $($rest)*] }};\n"
                    );
                    if vname == "Fill" && type_name == "Space" {
                        format!(
                            "(@build $expr:expr, {field}: Fill, $($rest:tt)*) => {{ {macro_name}![@build $expr.{field}(kairos::Space::Fill(1u16)), $($rest)*] }};\n{}",
                            base
                        )
                    } else {
                        base
                    }
                },
                VariantKind::QualifiedIdent(inner_path) => format!(
                    "(@build $expr:expr, {field}: {vname}($inner:ident), $($rest:tt)*) => {{ {macro_name}![@build $expr.{field}({path}::{vname}({inner_path}::$inner)), $($rest)*] }};\n"
                ),
                VariantKind::ScaleShorthand => {
                    if type_name == "Space" {
                        format!(
                            "(@build $expr:expr, {field}: {vname}, $($rest:tt)*) => {{ {macro_name}![@build $expr.{field}(kairos::Space::Scale(kairos::Size::{vname})), $($rest)*] }};\n"
                        )
                    } else if type_name == "Size" {
                        format!(
                            "(@build $expr:expr, {field}: {vname}, $($rest:tt)*) => {{ {macro_name}![@build $expr.{field}(kairos::Size::{vname}), $($rest)*] }};\n"
                        )
                    } else {
                        String::new()
                    }
                },
            };
            arms.push_str(&arm);
        }
    }

    arms
}