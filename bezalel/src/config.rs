use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub github_token: Option<String>,
}

pub fn config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".bezalel")
        .join("config.toml")
}

pub fn load() -> Config {
    let path = config_path();
    if !path.exists() {
        return Config::default();
    }
    let contents = std::fs::read_to_string(&path)
        .expect("Could not read config file");
    toml::from_str(&contents).unwrap_or_default()
}

pub fn save(config: &Config) {
    let path = config_path();
    std::fs::create_dir_all(path.parent().unwrap())
        .expect("Could not create config directory");
    let contents = toml::to_string(config)
        .expect("Could not serialize config");
    std::fs::write(&path, contents)
        .expect("Could not write config file");
}

pub fn get_or_prompt_token() -> String {
    let mut config = load();
    if let Some(token) = config.github_token.clone() {
        return token;
    }

    println!("No GitHub token found.");
    println!("Generate one at: https://github.com/settings/tokens");
    println!("Required permissions: repo, workflow");
    print!("Enter token: ");

    let mut token = String::new();
    std::io::stdin().read_line(&mut token).unwrap();
    let token = token.trim().to_string();

    config.github_token = Some(token.clone());
    save(&config);
    println!("Token saved to ~/.bezalel/config.toml");

    token
}