use mantle::types::theme::{ThemeSet, ThemeVariant};
use petra::{Component, InputEvent};
use crate::conversion::{to_iced, KairosEvent};

pub trait App: Component {
    fn title(&self) -> &str;
    fn theme(&self) -> &ThemeSet;
    fn variant(&self) -> ThemeVariant;
    fn icon(&self) -> Option<&'static [u8]> { None }
    fn min_size(&self) -> Option<(f32, f32)> { None }
    fn tick_rate(&self) -> u64 { 8 }
}

struct KairosRunner<A: App> {
    root: A,
    version: u64,
    time: f32,
    cursor_pos: std::sync::Arc<std::sync::Mutex<(f32, f32)>>,
}

pub fn run<A: App + 'static>(root: A) {
    let cursor_pos = std::sync::Arc::new(std::sync::Mutex::new((0.0f32, 0.0f32)));
    crate::CURSOR_POS.set(cursor_pos.clone()).ok();
    let runner = KairosRunner { root, version: 0, time: 0.0, cursor_pos };
    let icon = runner.root.icon()
        .and_then(|bytes| iced::window::icon::from_file_data(bytes, None).ok());
    let min_size = runner.root.min_size()
        .map(|(w, h)| iced::Size::new(w, h));

    let mut app = iced::application(
        |r: &KairosRunner<A>| r.root.title().to_string(),
        KairosRunner::update,
        KairosRunner::view,
    );
    app = app.window(iced::window::Settings {
        icon,
        min_size,
        ..Default::default()
    });
    let app = app.subscription(KairosRunner::subscription);
    app.run_with(|| (runner, iced::Task::none())).unwrap();
}

impl<A: App> KairosRunner<A> {
    fn update(&mut self, event: KairosEvent) {
        self.version += 1;
        match event {
            KairosEvent::Press(f) => {
                if let Ok(mut callback) = f.lock() {
                    callback();
                }
            },
            KairosEvent::StringInput(sig, val) => sig.set(val),
            KairosEvent::Tick => {
                self.time += 1.0 / 120.0;
            },
            KairosEvent::None => {},
            KairosEvent::InputPress(f, x, y) => {
                if let Ok(mut handler) = f.lock() {
                    handler(InputEvent::Click { x, y });
                }
            },
            KairosEvent::KeyPress(key) => {
                if let Ok(mut handlers) = crate::key_handlers().lock() {
                    if let Some(handler) = handlers.get_mut(&key) {
                        if let Ok(mut h) = handler.lock() {
                            h(InputEvent::KeyPress(petra::Key(key)));
                        }
                    }
                }
            },
        }
    }

    fn subscription(&self) -> iced::Subscription<KairosEvent> {
        let tick = iced::time::every(std::time::Duration::from_millis(self.root.tick_rate()))
            .map(|_| KairosEvent::Tick);
        let keys = iced::keyboard::on_key_press(|key, _modifiers| {
            let key_str = format!("{:?}", key);
            Some(KairosEvent::KeyPress(key_str))
        });
        iced::Subscription::batch([tick, keys])
    }

    fn view(&self) -> iced::Element<'_, KairosEvent> {
        let theme = self.root.theme();
        let variant = self.root.variant();
        let palette = match variant {
            ThemeVariant::Dark => &theme.dark.palette,
            ThemeVariant::Light => &theme.light.palette,
        };
        let root_element = to_iced(self.root.build(palette), theme, variant, self.time);
        let cursor = self.cursor_pos.clone();
        iced::widget::mouse_area(root_element)
            .on_move(move |point| {
                *cursor.lock().unwrap() = (point.x, point.y);
                KairosEvent::None
            })
            .into()
    }
}