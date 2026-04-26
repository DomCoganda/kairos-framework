use image::DynamicImage;

pub fn render(path: &str) -> DynamicImage {
    let svg_data = std::fs::read(path).expect("Could not read SVG file");
    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(&svg_data, &opt).expect("Could not parse SVG");
    let size = tree.size().to_int_size();
    let width = size.width();
    let height = size.height();
    let mut pixmap = tiny_skia::Pixmap::new(width, height).expect("Could not create pixmap");
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let rgba = image::RgbaImage::from_raw(width, height, pixmap.take())
        .expect("Could not create image");
    image::DynamicImage::ImageRgba8(rgba)

}