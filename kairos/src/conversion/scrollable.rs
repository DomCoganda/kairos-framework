use petra::Widget;
use mantle::types::theme::{ThemeSet, ThemeVariant};
use crate::conversion::{to_iced, KairosEvent};

pub(crate) fn convert_scrollable(inner: Box<Widget>, theme: &ThemeSet, variant: ThemeVariant, time:f32) -> iced::Element<'static, KairosEvent> {
    let content = to_iced(*inner, theme, variant, time);
    iced::widget::scrollable(content).into()
}