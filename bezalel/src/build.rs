use crate::config;
use crate::github::GitHub;
use std::thread;
use std::time::Duration;

pub fn run(app: &str, owner: &str, repo: &str) {
    let binary_name = read_binary_name(app);
    let app_id = format!("org.seraph.{}", capitalize(app));
    crate::workflow::generate(app, &binary_name);
    crate::workflow::generate_flatpak_manifest(app, &binary_name, &app_id);
    let token = config::get_or_prompt_token();
    let gh = GitHub::new(token, owner.to_string(), repo.to_string());
    let workflow = "release.yml";

    println!("Triggering build for {}...", app);
    if let Err(e) = gh.trigger_workflow(workflow) {
        eprintln!("Failed to trigger workflow: {}", e);
        return;
    }

    println!("Waiting for workflow to start...");
    thread::sleep(Duration::from_secs(5));

    let run_id = match gh.get_latest_run_id(workflow) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to get run ID: {}", e);
            return;
        }
    };

    println!("Polling build status...");
    loop {
        match gh.poll_run(run_id) {
            Ok((status, conclusion)) if status == "completed" => {
                if conclusion == "success" {
                    println!("✓ Build complete");
                } else {
                    eprintln!("✗ Build failed with conclusion: {}", conclusion);
                    eprintln!("Check https://github.com/{}/{}/actions for details", owner, repo);
                    return;
                }
                break;
            }
            Ok((status, _)) => {
                println!("  status: {}...", status);
                thread::sleep(Duration::from_secs(10));
            }
            Err(e) => {
                eprintln!("Failed to poll run: {}", e);
                return;
            }
        }
    }

    println!("Downloading binaries...");
    if let Err(e) = gh.download_artifacts(run_id, "dist") {
        eprintln!("Failed to download artifacts: {}", e);
        return;
    }

    println!("✓ All binaries saved to dist/");
}

fn read_binary_name(fallback: &str) -> String {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").unwrap_or_default();
    cargo_toml
        .lines()
        .find(|l| l.trim_start().starts_with("name"))
        .and_then(|l| l.split('"').nth(1))
        .unwrap_or(fallback)
        .to_string()
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}