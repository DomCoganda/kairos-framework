use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef>{
    vec![
        TypeDef {
            full_path: "kairos :: PaletteFill",
            variants: vec![
                VariantDef { name: "Palette", kind: VariantKind::QualifiedIdent("kairos :: PaletteColor") },
                VariantDef { name: "Custom",  kind: VariantKind::OpaqueData },
                VariantDef { name: "Surface", kind: VariantKind::OpaqueData },
            ],
        },
    ]
}