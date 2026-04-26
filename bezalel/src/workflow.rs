pub fn generate(app: &str, binary_name: &str) {
    let workflow_dir = ".github/workflows";
    let workflow_path = format!("{}/release.yml", workflow_dir);

    if std::path::Path::new(&workflow_path).exists() {
        return;
    }

    std::fs::create_dir_all(workflow_dir)
        .expect("Could not create .github/workflows directory");

    let app_id = format!("org.seraph.{}{}",
                         &binary_name[..1].to_uppercase(),
                         &binary_name[1..]);

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
      - name: Install flatpak-builder
        run: |
          sudo apt-get update
          sudo apt-get install -y flatpak flatpak-builder
          flatpak remote-add --if-not-exists --user flathub https://flathub.org/repo/flathub.flatpakrepo
          flatpak install --user -y flathub org.freedesktop.Platform//23.08 org.freedesktop.Sdk//23.08
          flatpak install --user -y flathub org.freedesktop.Sdk.Extension.rust-stable//23.08
      - name: Build Flatpak
        run: flatpak-builder --user --install --force-clean build-dir {app_id}.json
      - name: Export Flatpak bundle
        run: flatpak build-bundle ~/.local/share/flatpak/repo {app}-linux.flatpak {app_id}
      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: {app}-linux
          path: {app}-linux.flatpak

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
pub fn generate_flatpak_manifest(app: &str, app_id: &str) {
    let manifest_path = format!("{}.json", app_id);

    if std::path::Path::new(&manifest_path).exists() {
        return;
    }

    let contents = format!(r#"{{
      "app-id": "{app_id}",
      "runtime": "org.freedesktop.Platform",
      "runtime-version": "23.08",
      "sdk": "org.freedesktop.Sdk",
      "sdk-extensions": ["org.freedesktop.Sdk.Extension.rust-stable"],
      "command": "{app}",
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
          "CARGO_HOME": "/run/build/{app}/cargo"
        }}
      }},
      "modules": [
        {{
          "name": "{app}",
          "buildsystem": "simple",
          "build-commands": [
            "cargo build --release",
            "install -Dm755 target/release/{app} /app/bin/{app}"
          ],
          "sources": [
            {{
              "type": "dir",
              "path": "."
            }}
          ]
        }}
      ]
    }}"#, app = app, app_id = app_id);

    std::fs::write(&manifest_path, contents)
        .expect("Could not write Flatpak manifest");

    println!("✓ Generated {}", manifest_path);
}