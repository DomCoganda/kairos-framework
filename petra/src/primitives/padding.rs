use crate::primitives::types::Space;


///Defines how the axis for padding should work are they symmetrical or different to each other
pub enum AxisPadding {
    Symmetrical(Space),
    Custom(Space, Space),
}

///Defines how the structure of padding works in seraphUi
#[derive(Clone)]
pub struct Padding {
    pub left: Space,
    pub right: Space,
    pub top: Space,
    pub bottom: Space,
}

impl Padding {
    ///Makes it easier for users to have the same value for padding on all the sides
    pub fn all(size: Space)   ->
    Padding {
        Padding{
            top: size,
            bottom: size,
            left: size,
            right: size,
        }
    }

    ///Makes it easier for users to have same padding size or different padding sizes of the horizontal axis
    pub fn horizontal(axis: AxisPadding)   ->
    Padding {
        let (left, right) = match axis {
            AxisPadding::Symmetrical(size) => (size, size),
            AxisPadding::Custom(left, right) => (left, right),
        };
        Padding{
            top: Space::Custom(0.0),
            bottom: Space::Custom(0.0),
            left,
            right,
        }
    }

    ///Makes it easier for users to have a border only on the top and bottom
    pub fn vertical(axis: AxisPadding) ->
    Padding {
        let (top, bottom) = match axis {
            AxisPadding::Symmetrical(size) => (size, size),
            AxisPadding::Custom(top, bottom) => (top, bottom)
        };
        Padding{
            top,
            bottom,
            left: Space::Custom(0.0),
            right: Space::Custom(0.0),
        }
    }

    ///Makes it easier for user to have the same horizontal and vertical padding or different padding sizes based on the axis
    pub fn symmetric(horizontal: AxisPadding, vertical: AxisPadding) -> Padding {
        let (top, bottom) = match vertical {
            AxisPadding::Symmetrical(size) => (size, size),
            AxisPadding::Custom(top, bottom) => (top, bottom)
        };
        let (left, right) = match horizontal{
            AxisPadding::Symmetrical(size) => (size, size),
            AxisPadding::Custom(left, right) => (left, right),
        };
        Padding{
            top,
            bottom,
            left,
            right,
        }
    }

    ///Makes it easier for user to have no border
    pub fn none() -> Padding {
        Padding{
            top: Space::Custom(0.0),
            bottom: Space::Custom(0.0),
            left: Space::Custom(0.0),
            right: Space::Custom(0.0),
        }
    }
}