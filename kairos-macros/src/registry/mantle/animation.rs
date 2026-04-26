use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef>{
    vec![
        TypeDef{
            full_path: "kairos :: Animation",
            variants: vec![
                VariantDef{ name: "Slow", kind: VariantKind::Unit },
                VariantDef{ name: "Normal", kind: VariantKind::Unit },
                VariantDef{ name: "Fast", kind: VariantKind::Unit },
            ]
        }
    ]
}