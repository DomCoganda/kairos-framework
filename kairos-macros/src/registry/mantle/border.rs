use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef>{
    vec![
        TypeDef{
            full_path: "kairos :: BorderRadius",
            variants: vec![
                VariantDef{ name: "Sharp", kind: VariantKind::Unit },
                VariantDef{ name: "Pill", kind: VariantKind::Unit },
                VariantDef{ name: "Rounded", kind: VariantKind::OpaqueData },
            ]
        },
        TypeDef {
            full_path: "kairos :: BorderSides",
            variants: vec![
                VariantDef { name: "All",        kind: VariantKind::Unit },
                VariantDef { name: "None",       kind: VariantKind::Unit },
                VariantDef { name: "Horizontal", kind: VariantKind::Unit },
                VariantDef { name: "Vertical",   kind: VariantKind::Unit },
            ],
        },
    ]
}