use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef> {
    vec![
        TypeDef {
            full_path: "kairos :: Space",
            variants: vec![
                VariantDef { name: "Fill", kind: VariantKind::OpaqueData },
                VariantDef { name: "Shrink", kind: VariantKind::Unit },
                VariantDef { name: "Custom", kind: VariantKind::OpaqueData },
                VariantDef { name: "Scale",  kind: VariantKind::OpaqueData },
                VariantDef { name: "Xs", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Sm", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Md", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Lg", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Xl", kind: VariantKind::ScaleShorthand },
            ],
        },
        TypeDef {
            full_path: "kairos :: Size",
            variants: vec![
                VariantDef { name: "Xs", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Sm", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Md", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Lg", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Xl", kind: VariantKind::ScaleShorthand },
                VariantDef { name: "Custom", kind: VariantKind::OpaqueData },
            ],
        },
    ]
}