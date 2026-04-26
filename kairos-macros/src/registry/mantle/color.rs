use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef> {
    vec![
        TypeDef {
            full_path: "kairos :: PaletteColor",
            variants: vec![
                VariantDef { name: "Primary",    kind: VariantKind::Unit },
                VariantDef { name: "Secondary",  kind: VariantKind::Unit },
                VariantDef { name: "Accent",     kind: VariantKind::Unit },
                VariantDef { name: "Text",       kind: VariantKind::Unit },
                VariantDef { name: "Background", kind: VariantKind::Unit },
                VariantDef { name: "Error",      kind: VariantKind::Unit },
                VariantDef { name: "Warning",    kind: VariantKind::Unit },
                VariantDef { name: "Success",    kind: VariantKind::Unit },
            ],
        },
        TypeDef {
            full_path: "kairos :: ColorSource",
            variants: vec![
                VariantDef { name: "Custom",  kind: VariantKind::OpaqueData },
                VariantDef { name: "Palette", kind: VariantKind::QualifiedIdent("kairos :: PaletteColor") },
            ],
        },
        TypeDef {
            full_path: "kairos :: Fill",
            variants: vec![
                VariantDef { name: "Surface", kind: VariantKind::Unit },
                VariantDef { name: "Palette", kind: VariantKind::QualifiedIdent("kairos :: PaletteColor") },
                VariantDef { name: "Custom",  kind: VariantKind::OpaqueData },
            ],
        },
    ]
}