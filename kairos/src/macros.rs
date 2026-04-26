#[macro_export]
macro_rules! children {
    ($($child:expr),* $(,)?) => {
        vec![$(
            $crate::Widget::Deferred(std::sync::Arc::new({
                let c = $child;
                move |p| $crate::Buildable::resolve(c.clone(), p)
            }))
        ),*]
    }
}

#[macro_export]
macro_rules! switcher {
    (signal: $signal:expr, view![$($pat:pat => $body:expr),* $(,)?]) => {{
        Switcher::new(
            $signal.clone(),
            std::sync::Arc::new(move |value| match value {
                $($pat => $body),*
            })
        )
    }};
}

#[macro_export]
macro_rules! clickable_insert {
    // base case — nothing left
    ($map:ident) => {};

    // MouseButton, no use_cords
    ($map:ident MouseButton($btn:ident) => { $($body:tt)* }) => {
        $map.insert(
            $crate::Binding::Mouse($crate::MouseButton::$btn),
            Box::new(move |_: $crate::InputEvent| { $($body)* })
                as Box<dyn FnMut($crate::InputEvent) + Send + 'static>,
        );
    };

    ($map:ident MouseButton($btn:ident) =>  { $($body:tt)* } , $($rest:tt)*) => {
        $map.insert(
            $crate::Binding::Mouse($crate::MouseButton::$btn),
            Box::new(move |e: $crate::InputEvent| {
                let $crate::InputEvent::Click { x, y } = e else { return; };
                $($body)*
            }) as Box<dyn FnMut($crate::InputEvent) + Send + 'static>,
        );
        $crate::clickable_insert!($map $($rest)*);
    };

    // MouseButton, use_cords, with comma
    ($map:ident MouseButton($btn:ident) => ($x:ident, $y:ident) { $($body:tt)* } , $($rest:tt)*) => {
        $map.insert(
            $crate::Binding::Mouse($crate::MouseButton::$btn),
            Box::new(move |_: $crate::InputEvent| {
                let $x = $crate::cursor_pos().0;
                let $y = $crate::cursor_pos().1;
                $($body)*
            }) as Box<dyn FnMut($crate::InputEvent) + Send + 'static>,
        );
        $crate::clickable_insert!($map $($rest)*);
    };

    // MouseButton, use_cords, last binding (no comma)
    ($map:ident MouseButton($btn:ident) => ($x:ident, $y:ident) { $($body:tt)* }) => {
        $map.insert(
            $crate::Binding::Mouse($crate::MouseButton::$btn),
            Box::new(move |_: $crate::InputEvent| {
                let (x, y) = $crate::cursor_pos();
                $($body)*
            }) as Box<dyn FnMut($crate::InputEvent) + Send + 'static>,
        );
    };

    // Key
    ($map:ident Key($key:literal) => { $($body:tt)* } $(,)? $($rest:tt)*) => {
        $map.insert(
            $crate::Binding::Key($crate::Key($key.to_string())),
            Box::new(move  |_: $crate::InputEvent| { $($body)* })
                as Box<dyn FnMut($crate::InputEvent) + Send + 'static>,
        );
        $crate::clickable_insert!($map $($rest)*);
    };
}

#[macro_export]
macro_rules! clickable {
    (bindings: [ $($binding:tt)* ], $child:expr) => {{
        let mut map = std::collections::HashMap::new();
        $crate::clickable_insert!(map $($binding)*);
        $crate::clickable(Box::new($child), map)
    }};
}

#[macro_export]
macro_rules! deferred {
    ($($body:tt)*) => {
        Widget::Deferred(std::sync::Arc::new(move |palette| {
            $($body)*
        }))
    }
}

#[macro_export]
macro_rules! border {
    ($($field:ident: $val:expr),* $(,)?) => {
        $crate::Border::new()
            $(.$field($val))*
    }
}

#[macro_export]
macro_rules! positioned {
    (@build $expr:expr,) => { $expr };
    (@build $expr:expr, width: Fill, $($rest:tt)*) => { positioned![@build $expr.width(kairos::Space::Fill), $($rest)*] };
    (@build $expr:expr, width: Shrink, $($rest:tt)*) => { positioned![@build $expr.width(kairos::Space::Shrink), $($rest)*] };
    (@build $expr:expr, height: Fill, $($rest:tt)*) => { positioned![@build $expr.height(kairos::Space::Fill), $($rest)*] };
    (@build $expr:expr, height: Shrink, $($rest:tt)*) => { positioned![@build $expr.height(kairos::Space::Shrink), $($rest)*] };
    (@build $expr:expr, $field:ident: $val:expr, $($rest:tt)*) => { positioned![@build $expr.$field($val), $($rest)*] };
    ($($tokens:tt)*) => { positioned![@build kairos::Positioned::new(), $($tokens)*] };
}