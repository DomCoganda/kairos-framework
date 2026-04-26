use image::DynamicImage;

pub fn generate(img: &DynamicImage) {
    #[cfg(target_os = "windows")]
    windows(img);

    #[cfg(target_os = "linux")]
    linux(img);

    #[cfg(target_os = "macos")]
    macos(img);
}

#[cfg(target_os = "windows")]
fn windows(img: &DynamicImage) {
    img.resize(256, 256, image::imageops::FilterType::Lanczos3)
        .save("icon.ico")
        .expect("Could not save .ico");
    winres::WindowsResource::new()
        .set_icon("icon.ico")
        .compile()
        .expect("Could not embed icon");
}

#[cfg(target_os = "linux")]
fn linux(img: &DynamicImage) {
    for size in [16, 32, 48, 64, 128, 256] {
        img.resize(size, size, image::imageops::FilterType::Lanczos3)
            .save(format!("icon_{}.png", size))
            .expect("Could not save PNG");
    }
}

#[cfg(target_os = "macos")]
fn macos(img: &DynamicImage) {
    img.resize(1024, 1024, image::imageops::FilterType::Lanczos3)
        .save("icon.png")
        .expect("Could not save macOS icon");
}