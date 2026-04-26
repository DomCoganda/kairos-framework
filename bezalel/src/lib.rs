mod svg;
mod icons;

pub fn build() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let svg_path = format!("{}/src/assets/logo.svg", manifest_dir);
    let png = svg::render(&svg_path);
    icons::generate(&png);
}