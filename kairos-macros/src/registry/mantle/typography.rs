use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef>{
    vec![
        TypeDef{
            full_path: "kairos :: TextStyle",
            variants: vec![
                VariantDef{ name: "H1", kind: VariantKind::Unit },
                VariantDef{ name: "H2", kind: VariantKind::Unit },
                VariantDef{ name: "H3", kind: VariantKind::Unit },
                VariantDef{ name: "H4", kind: VariantKind::Unit },
                VariantDef{ name: "Body", kind: VariantKind::Unit },
                VariantDef{ name: "Caption", kind: VariantKind::Unit },
                VariantDef{ name: "Label", kind: VariantKind::Unit },
                VariantDef{ name: "Code", kind: VariantKind::Unit },
            ]
        }
    ]
}