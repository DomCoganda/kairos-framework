pub mod mantle;
pub mod petra;

pub enum VariantKind {
    Unit,
    OpaqueData,
    QualifiedIdent(&'static str),
    ScaleShorthand,
}

pub struct VariantDef {
    pub name: &'static str,
    pub kind: VariantKind,
}

pub struct TypeDef {
    pub full_path: &'static str,
    pub variants: Vec<VariantDef>,
}

pub fn build_registry() -> Vec<TypeDef> {
    let mut all = Vec::new();
    all.extend(mantle::all());
    all.extend(petra::all());
    all
}