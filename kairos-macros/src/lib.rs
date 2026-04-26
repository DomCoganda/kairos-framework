mod generate;
mod registry;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    generate::generate(&input, false).into()
}

#[proc_macro_attribute]
pub fn primitive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    generate::generate(&input, true).into()
}

#[proc_macro]
pub fn icon_png(input: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(input as syn::LitStr);
    let path = path_lit.value();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let full_path = std::path::Path::new(&manifest_dir).join(&path);

    let svg_data = std::fs::read(&full_path)
        .unwrap_or_else(|_| panic!("could not read svg: {}", full_path.display()));

    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(&svg_data, &options)
        .unwrap_or_else(|e| panic!("could not parse svg: {}", e));

    let width = tree.size().width() as u32;
    let height = tree.size().height() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .expect("could not create pixmap");

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let png_bytes = pixmap.encode_png()
        .expect("could not encode png");

    let bytes_lit = png_bytes.iter().map(|b| quote::quote!(#b));

    quote::quote!(&[#(#bytes_lit),*]).into()
}