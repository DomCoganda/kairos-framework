pub mod color;
pub mod sizing;
pub mod typography;
pub mod border;
pub mod visual;
pub mod animation;
pub mod icons;
pub mod line;

use crate::registry::TypeDef;

pub fn all() -> Vec<TypeDef> {
    let mut all = Vec::new();
    all.extend(color::types());
    all.extend(sizing::types());
    all.extend(typography::types());
    all.extend(border::types());
    all.extend(visual::types());
    all.extend(animation::types());
    all.extend(line::types());
    all.extend(icons::types());
    all
}