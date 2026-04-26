use kairos_macros::primitive;
use crate::primitives::types::*;
use crate::primitives::padding::Padding;
use crate:: Widget;
use mantle::palette::Fill as BackgroundFill;
use crate::primitives::positioned::Positioned;
use crate::Space::Fill;

///Defines the structure of a stack in SeraphUi
#[primitive]
pub struct Stack {
   pub width: Space,
   pub height: Space,
   pub padding: Padding,
   pub alignment: ( HorizontalAlignment, VerticalAlignment ),
   pub background: Option<BackgroundFill>,
   pub children: Vec<Widget>,
   pub offset: (f32, f32),
   #[push]
   pub positioned: Vec<Positioned>,
}

impl Stack {
   pub fn new() -> Self {
      Stack{
         width: Fill(1),
         height: Fill(1),
         padding: Padding::none(),
         alignment: (HorizontalAlignment::Left, VerticalAlignment::Top),
         background: None,
         children: vec![],
         offset: (0.0, 0.0),  
         positioned: vec![],

      }

   }
}