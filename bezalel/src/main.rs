mod config;
mod github;
mod build;
mod svg;
mod icons;
mod workflow;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let app = args.get(2).map(|s| s.as_str()).unwrap_or("");

    match command {
        "run" => {
            if app.is_empty() {
                println!("Usage: bezalel run <app-name> <owner> <repo>");
                return;
            }
            let owner = args.get(3).map(|s| s.as_str()).unwrap_or("");
            let repo = args.get(4).map(|s| s.as_str()).unwrap_or("");
            if owner.is_empty() || repo.is_empty() {
                println!("Usage: bezalel run <app-name> <owner> <repo>");
                return;
            }
            build::run(app, owner, repo);
        }
        _ => {
            println!("Usage: bezalel run <app-name> <owner> <repo>");
        }
    }
}