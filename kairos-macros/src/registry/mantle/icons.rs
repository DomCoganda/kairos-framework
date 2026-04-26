use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef> {
    vec![
        TypeDef {
            full_path: "kairos :: IconStyle",
            variants: vec![
                VariantDef { name: "Filled", kind: VariantKind::Unit },
                VariantDef { name: "Outline ", kind: VariantKind::Unit },
            ],
        },
    ]
}