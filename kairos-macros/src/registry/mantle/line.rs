use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef> {
    vec![
        TypeDef {
            full_path: "kairos :: LineStyle",
            variants: vec![
                VariantDef { name: "Solid", kind: VariantKind::Unit },
                VariantDef { name: "Repeated ", kind: VariantKind::OpaqueData },
            ],
        },
    ]
}