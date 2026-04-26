use crate::types::theme::ThemeSet;
use std::path::Path;

pub fn load(path: &Path) -> Result<ThemeSet, String> {
    let contents = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let theme = toml::from_str(&contents).map_err(|error| error.to_string())?;
    Ok(theme)
}

pub fn save(theme: &ThemeSet, path: &Path) -> Result<(), String> {
    let contents = toml::to_string(theme).map_err(|error| error.to_string())?;
    std::fs::write(path, contents).map_err(|error| error.to_string())?;
    Ok(())
}