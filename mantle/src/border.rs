use serde::{Serialize, Deserialize};
use crate::color::*;
use crate::line::LineStyle;

/// Represents the corner radius of a border

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum BorderRadius {
    Sharp,
    Rounded(f32),
    Pill,
}

///Defines the border sides shown in SeraphUi
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum BorderSides {
    All,
    None,
    Horizontal,
    Vertical,
    Custom { top: bool, bottom: bool, left: bool, right: bool },
}

/// Represents the entire border structure that a component can use
#[derive(Serialize, Deserialize, Clone)]
pub struct Border {
    pub thickness: f32,
    pub color: ColorSource,
    pub sides: BorderSides,
    pub style: LineStyle,
    pub radius: BorderRadius,
}

///Fre Form Functions to make it easier to build Borders

///makes it easier for user to change the color of a border in seraphUi.
impl Border {

    ///Makes it easier fo users to change the border color in seraphUi
    pub fn color(mut self, value: impl Into<ColorSource>) -> Self {
        self.color = value.into();
        self
    }

    ///makes it easier for user to choose which of the border sides of a border show in seraphUi
    pub fn sides( mut self, value: BorderSides) -> Self {
        self.sides= value;
        self
    }

    //makes it easier for users to change the line of the border in seraphUi
    pub fn style( mut self, value: LineStyle) -> Self {
        self.style= value;
        self
    }

    ///makes it easier for users to change the border thickness in seraphUi
    pub fn thickness( mut self, value: f32) -> Self {
        self.thickness= value;
        self
    }

    ///makes it easier for users to change the radius of a border in seraphUi
    pub fn radius( mut self, value: BorderRadius) -> Self {
        self.radius= value;
        self
    }
    ///Defines the base function of how users create borders in seraphUi
    pub fn new () -> Self {
        Border { thickness: 0.0,
            color: ColorSource::Custom(Color::new("#000000", 0.0)),
            sides: BorderSides::None,
            style: LineStyle::Solid,
            radius: BorderRadius::Sharp,
        }
    }

    ///makes it easier for user to have no border in graphical
    pub fn none() -> Self {
        Border::new()
    }
    /*
        ///make the default style of a button in seraphUi.
        pub fn default () -> Self {
            Border::new()
                .thickness(0.0)
                .color(Secondary )
                .sides(all())
                .style(solid())
                .radius(Rounded(5.0))
        }
    */
}