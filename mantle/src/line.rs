use serde::{Deserialize, Serialize};
use crate::shape::Shape;

/// Represents how any line in seraphUi looks
#[derive(Serialize, Deserialize, Clone)]
pub enum LineStyle {
    Solid,
    Repeated(Shape, u8),
}