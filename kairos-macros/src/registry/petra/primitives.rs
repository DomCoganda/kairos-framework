use crate::registry::{TypeDef, VariantDef, VariantKind};

pub fn types() -> Vec<TypeDef> {
    vec![
        TypeDef {
            full_path: "kairos :: Space",
            variants: vec![
                VariantDef { name: "Scale",  kind: VariantKind::OpaqueData },
                VariantDef { name: "Custom", kind: VariantKind::OpaqueData },
                VariantDef { name: "Fill", kind: VariantKind::OpaqueData },
                VariantDef { name: "Shrink", kind: VariantKind::Unit },
            ],
        },
        TypeDef {
            full_path: "kairos :: HorizontalAlignment",
            variants: vec![
                VariantDef { name: "Left",    kind: VariantKind::Unit },
                VariantDef { name: "Center",  kind: VariantKind::Unit },
                VariantDef { name: "Right",   kind: VariantKind::Unit },
                VariantDef { name: "Stretch", kind: VariantKind::Unit },
            ],
        },
        TypeDef {
            full_path: "kairos :: VerticalAlignment",
            variants: vec![
                VariantDef { name: "Top",     kind: VariantKind::Unit },
                VariantDef { name: "Center",  kind: VariantKind::Unit },
                VariantDef { name: "Bottom",  kind: VariantKind::Unit },
                VariantDef { name: "Stretch", kind: VariantKind::Unit },
            ],
        },
        TypeDef {
            full_path: "kairos :: Orientation",
            variants: vec![
                VariantDef { name: "Horizontal", kind: VariantKind::Unit },
                VariantDef { name: "Vertical",   kind: VariantKind::Unit },
            ],
        },
        TypeDef {
            full_path: "kairos :: Position",
            variants: vec![
                VariantDef { name: "TopLeft",     kind: VariantKind::Unit },
                VariantDef { name: "TopCenter",   kind: VariantKind::Unit },
                VariantDef { name: "TopRight",    kind: VariantKind::Unit },
                VariantDef { name: "BottomLeft",  kind: VariantKind::Unit },
                VariantDef { name: "BottomCenter",kind: VariantKind::Unit },
                VariantDef { name: "BottomRight", kind: VariantKind::Unit },
            ],
        },
        TypeDef {
            full_path: "kairos :: SpinnerShape",
            variants: vec![
                VariantDef { name: "Dot",      kind: VariantKind::Unit },
                VariantDef { name: "Line",     kind: VariantKind::Unit },
                VariantDef { name: "Arc",      kind: VariantKind::Unit },
                VariantDef { name: "Triangle", kind: VariantKind::Unit },
                VariantDef { name: "Square",   kind: VariantKind::Unit },
                VariantDef { name: "Custom",   kind: VariantKind::OpaqueData },
            ],
        },
        TypeDef {
            full_path: "kairos :: SpinnerMotion",
            variants: vec![
                VariantDef { name: "Ring",   kind: VariantKind::Unit },
                VariantDef { name: "Chase",  kind: VariantKind::Unit },
                VariantDef { name: "Spin",   kind: VariantKind::Unit },
                VariantDef { name: "Wave",   kind: VariantKind::Unit },
                VariantDef { name: "Grow",   kind: VariantKind::Unit },
                VariantDef { name: "Bounce", kind: VariantKind::Unit },
                VariantDef { name: "Orbit",  kind: VariantKind::Unit },
                VariantDef { name: "Pulse",  kind: VariantKind::Unit },
                VariantDef { name: "Comet",  kind: VariantKind::Unit },
                VariantDef { name: "Ripple", kind: VariantKind::Unit },
                VariantDef { name: "Morph",  kind: VariantKind::OpaqueData },
            ],
        },
        TypeDef {
            full_path: "kairos :: SpinnerDirection",
            variants: vec![
                VariantDef { name: "Forward", kind: VariantKind::Unit },
                VariantDef { name: "Reverse", kind: VariantKind::Unit },
            ],
        },
    ]
}