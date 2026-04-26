use mantle::visual::SurfaceStyle;

///Defines the structure of an overlay in seraphUi
pub struct Overlay<Dismiss> {
    pub background: Option<SurfaceStyle>,
    pub visible: bool,
    pub on_dismiss: Option<Dismiss>
}