use serde::{Serialize, Deserialize};
use crate::palette::*;
use crate::icons::*;
use crate::sizing::*;
use crate::border::*;
use crate::typography::*;
use crate::animation::*;

/// A single complete theme containing all visual properties for an app or the OS
#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub palette: Palette,
    pub icons: IconTheme,
    pub border: Option<Border>,
}

/// The two variants every types set must provide
#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum ThemeVariant {
    Light,
    Dark,
}

/// A paired set of light and dark themes
#[derive(Serialize, Deserialize)]
pub struct ThemeSet {
    pub light: Theme,
    pub dark: Theme,
    pub typography: Typography,
    pub sizes: SizeScale,
    pub animations: AnimationScale,
}