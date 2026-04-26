use serde::{Serialize, Deserialize};
/// Configurable size scale used for icons, spacing and padding
#[derive(Serialize, Deserialize)]
pub struct SizeScale {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

/// Named size variants used to reference a SizeScale value
#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Custom(f32),
}

impl Size {
    pub fn to_px(&self, scale: &SizeScale) -> f32 {
        match self {
            Size::Xs => scale.xs,
            Size::Sm => scale.sm,
            Size::Md => scale.md,
            Size::Lg => scale.lg,
            Size::Xl => scale.xl,
            Size::Custom(v) => *v,
        }
    }
}