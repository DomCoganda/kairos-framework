pub mod primitives;
pub mod composites;
pub mod helpers;

use crate::registry::TypeDef;

pub fn all() -> Vec<TypeDef> {
    let mut all = Vec::new();
    all.extend(primitives::types());
    all.extend(composites::types());
    all.extend(helpers::types());
    all
}