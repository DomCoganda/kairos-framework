use crate::animation::AnimationScale;
use crate::color::{Color, ColorSource};
use crate::icons::{IconStyle, IconTheme};
use crate::palette::{Palette, PaletteColor};
use crate::sizing::{Size, SizeScale};
use super::theme::{Theme, ThemeSet};
use crate::typography::{Font, FontStyle, FontWeight, Typography};
use crate::visual::SurfaceStyle;

impl ThemeSet {
    pub fn default() -> ThemeSet {
        ThemeSet {

            typography: Typography {
                family: "Geist".to_string(),
                mono_family: "JetBrains Mono".to_string(),
                h1: Font {
                    size: 32.0,
                    weight: FontWeight::Black,
                    style: FontStyle::Normal,
                    decoration: None,
                    shadow: None,
                },
                h2: Font {
                    size: 24.0,
                    weight: FontWeight::ExtraBold,
                    style: FontStyle::Normal,
                    decoration: None,
                    shadow: None,
                },
                h3: Font {
                    size: 20.0,
                    weight: FontWeight::Bold,
                    style: FontStyle::Normal,
                    decoration: None,
                    shadow: None,
                },
                h4: Font {
                    size: 16.0,
                    weight: FontWeight::Semibold,
                    style: FontStyle::Normal,
                    decoration: None,
                    shadow: None,
                },
                body: Font {
                    size: 14.0,
                    weight: FontWeight::Medium,
                    style: FontStyle::Normal,
                    decoration: None,
                    shadow: None,
                },
                caption: Font {
                    size: 11.0,
                    weight: FontWeight::ExtraLight,
                    style: FontStyle::Italic,
                    decoration: None,
                    shadow: None,
                },
                label: Font {
                    size: 13.0,
                    weight: FontWeight::Light,
                    style: FontStyle::Normal,
                    decoration: None,
                    shadow: None,
                },
                code: Font {
                    size: 13.0,
                    weight: FontWeight::Thin,
                    style: FontStyle::Italic,
                    decoration: None,
                    shadow: None,
                },
            },
            sizes: SizeScale {
                xs: 4.0,
                sm: 8.0,
                md: 16.0,
                lg: 24.0,
                xl: 32.0,
            },
            animations: AnimationScale {
                slow: 400.0,
                normal: 200.0,
                fast: 100.0,
            },
            light: Theme {
                palette: Palette {
                    background: Color::new("#F0F4F8",1.0),
                    surface: SurfaceStyle::Solid(ColorSource::Custom(Color::new("#FFFFFF",1.0))),
                    primary: Color::new("#1E3A5F",1.0),
                    secondary: Color::new("#8A9EAF",1.0),
                    text: Color::new("#0A0A1A",1.0),
                    accent: Color::new("#FF5F1F",1.0),
                    error: Color::new("#D93025",1.0),
                    warning: Color::new("#F5A623",1.0),
                    success: Color::new("#1E8C45",1.0),
                },
                icons: IconTheme {
                    style: IconStyle::Filled,
                    color: ColorSource::Palette(PaletteColor::Accent),
                    size: Size::Md,
                },
                border: None,
            },
            dark: Theme {
                palette: Palette {
                    background: Color::new("#000020",1.0),
                    surface: SurfaceStyle::Solid(ColorSource::Custom(Color::new("#1A1A3A",1.0))),
                    primary: Color::new("#1E3A5F",1.0),
                    secondary: Color::new("#8A9EAF",1.0),
                    text: Color::new("#F2F2F7",1.0),
                    accent: Color::new("#FF5F1F",1.0),
                    error: Color::new("#FF453A",1.0),
                    warning: Color::new("#FFD60A",1.0),
                    success: Color::new("#32D74B",1.0),
                },
                icons: IconTheme {
                    style: IconStyle::Filled,
                    color: ColorSource::Palette(PaletteColor::Accent),
                    size: Size::Md,
                },
                border: None,
            }
        }
    }
}