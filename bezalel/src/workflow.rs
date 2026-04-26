pub fn generate(app: &str, binary_name: &str) {
    let workflow_dir = ".github/workflows";
    let workflow_path = format!("{}/release.yml", workflow_dir);

    if std::path::Path::new(&workflow_path).exists() {
        return;
    }

    std::fs::create_dir_all(workflow_dir)
        .expect("Could not create .github/workflows directory");

    let app_id = format!("org.seraph.{}{}",
                         &app[..1].to_uppercase(),
                         &app[1..]);

    let contents = format!(r#"name: Release

on:
  workflow_dispatch:

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc
      - name: Build
        run: cargo build --release --target x86_64-pc-windows-msvc
      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: {app}-windows
          path: target/x86_64-pc-windows-msvc/release/{binary_name}.exe

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Vendor dependencies
        run: |
          cargo vendor vendor
          mkdir -p .cargo
          cat >> .cargo/config.toml << 'EOF'
          [source.crates-io]
          replace-with = "vendored-sources"
          [source.vendored-sources]
          directory = "vendor"
          EOF
      - name: Install flatpak-builder
        run: |
          sudo apt-get update
          sudo apt-get install -y flatpak flatpak-builder
          flatpak remote-add --if-not-exists --user flathub https://flathub.org/repo/flathub.flatpakrepo
          flatpak install --user -y flathub org.freedesktop.Platform//24.08 org.freedesktop.Sdk//24.08
          flatpak install --user -y flathub org.freedesktop.Sdk.Extension.rust-stable//24.08
      - name: Build Flatpak
        run: flatpak-builder --user --install --force-clean build-dir {app_id}.json
      - name: Export Flatpak bundle
        run: flatpak build-bundle ~/.local/share/flatpak/repo {binary_name}-linux.flatpak {app_id}
      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: {app}-linux
          path: {binary_name}-linux.flatpak

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-apple-darwin
      - name: Build
        run: cargo build --release --target aarch64-apple-darwin
      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: {app}-macos
          path: target/aarch64-apple-darwin/release/{binary_name}
"#, app = app, binary_name = binary_name, app_id = app_id);

    std::fs::write(&workflow_path, contents)
        .expect("Could not write release.yml");

    println!("✓ Generated .github/workflows/release.yml");
}

pub fn generate_flatpak_manifest(app: &str, binary_name: &str, app_id: &str) {
    let manifest_path = format!("{}.json", app_id);

    if std::path::Path::new(&manifest_path).exists() {
        return;
    }

    let contents = format!(r#"{{
      "app-id": "{app_id}",
      "runtime": "org.freedesktop.Platform",
      "runtime-version": "24.08",
      "sdk": "org.freedesktop.Sdk",
      "sdk-extensions": ["org.freedesktop.Sdk.Extension.rust-stable"],
      "command": "{binary_name}",
      "finish-args": [
        "--share=ipc",
        "--socket=wayland",
        "--socket=fallback-x11",
        "--device=dri",
        "--filesystem=home"
      ],
      "build-options": {{
        "append-path": "/usr/lib/sdk/rust-stable/bin",
        "env": {{
          "CARGO_HOME": "/run/build/{app}/cargo",
          "RUSTUP_TOOLCHAIN": "stable"
        }}
      }},
      "modules": [
        {{
          "name": "{app}",
          "buildsystem": "simple",
          "build-commands": [
            "mkdir -p .cargo && printf '[source.crates-io]\\nreplace-with = \"vendored-sources\"\\n[source.vendored-sources]\\ndirectory = \"vendor\"\\n' > .cargo/config.toml",
            "cargo build --release",
            "install -Dm755 target/release/{binary_name} /app/bin/{binary_name}"
          ],
          "sources": [
            {{
              "type": "dir",
              "path": "."
            }}
          ]
        }}
      ]
    }}"#, app = app, binary_name = binary_name, app_id = app_id);

    std::fs::write(&manifest_path, contents)
        .expect("Could not write Flatpak manifest");

    println!("✓ Generated {}", manifest_path);
}