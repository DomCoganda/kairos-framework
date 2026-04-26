use mantle::palette::Palette;
use mantle::color::{Color, ColorSource};
use mantle::visual::SurfaceStyle;

pub struct ActiveTheme<'a> {
    pub palette: &'a Palette,
}

impl<'a> ActiveTheme<'a> {
    pub fn background(&self) -> SurfaceStyle {
        SurfaceStyle::Solid(ColorSource::Custom(Color::new(&self.palette.background.hex, self.palette.background.alpha)))
    }

    pub fn primary(&self) -> SurfaceStyle {
        SurfaceStyle::Solid(ColorSource::Custom(Color::new(&self.palette.primary.hex, self.palette.primary.alpha)))
    }

    pub fn accent(&self) -> SurfaceStyle {
        SurfaceStyle::Solid(ColorSource::Custom(Color::new(&self.palette.accent.hex, self.palette.accent.alpha)))
    }
}