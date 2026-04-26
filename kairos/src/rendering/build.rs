#[cfg(feature = "build")]
pub fn generate_icon(svg_path: &str, out_path: &str) {
    use resvg::usvg;

    let svg_data = std::fs::read(svg_path)
        .unwrap_or_else(|_| panic!("could not read svg: {}", svg_path));

    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(&svg_data, &options)
        .unwrap_or_else(|e| panic!("could not parse svg: {}", e));

    let size = tree.size();
    let width = size.width() as u32;
    let height = size.height() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .expect("could not create pixmap");

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let out_dir = std::path::Path::new(out_path)
        .parent()
        .expect("invalid out_path");
    std::fs::create_dir_all(out_dir).ok();

    pixmap.save_png(out_path)
        .unwrap_or_else(|e| panic!("could not save png: {}", e));

    println!("cargo:rerun-if-changed={}", svg_path);
}