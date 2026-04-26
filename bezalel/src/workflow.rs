pub fn generate(app: &str, binary_name: &str) {
    let workflow_dir = ".github/workflows";
    let workflow_path = format!("{}/release.yml", workflow_dir);

    if std::path::Path::new(&workflow_path).exists() {
        return;
    }

    std::fs::create_dir_all(workflow_dir)
        .expect("Could not create .github/workflows directory");

    let contents = format!(r#"name: Release

on:
  workflow_dispatch:

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            name: {app}-windows
            binary: {binary_name}.exe
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            name: {app}-linux
            binary: {binary_name}
          - os: macos-latest
            target: aarch64-apple-darwin
            name: {app}-macos
            binary: {binary_name}

    runs-on: ${{{{ matrix.os }}}}

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{{{ matrix.target }}}}

      - name: Build
        run: cargo build --release --target ${{{{ matrix.target }}}}

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{{{ matrix.name }}}}
          path: target/${{{{ matrix.target }}}}/release/${{{{ matrix.binary }}}}
"#, app = app, binary_name = binary_name);

    std::fs::write(&workflow_path, contents)
        .expect("Could not write release.yml");

    println!("✓ Generated .github/workflows/release.yml");
}