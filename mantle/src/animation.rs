use serde::{Serialize, Deserialize};

/// Configurable animation speed scale in milliseconds
#[derive(Serialize, Deserialize)]
pub struct AnimationScale {
    pub slow: f32,
    pub normal: f32,
    pub fast: f32,
}

/// Named animation speed variants used to reference an AnimationScale value
#[derive(Serialize, Deserialize, Clone)]
pub enum Animation {
    Slow,
    Normal,
    Fast,
    Custom(f32),
}
