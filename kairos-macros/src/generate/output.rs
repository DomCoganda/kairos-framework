

use quote::quote;
use syn::DeriveInput;

pub fn build(
    input: &DeriveInput,
    clean_input: &DeriveInput,
    name: &syn::Ident,
    macro_name: &str,
    wrap_widget: bool,
    nested_arms: &str,
    icon_arms: &str,
    label_arms: &str,
    source_arms: &str,
    push_arms: &str,
    padding_arms: &str,
    field_rules: &str,
    registry_arms: &str,
    impl_methods: &str,
) -> proc_macro2::TokenStream {
    let impl_src = format!("impl {name} {{ {impl_methods} }}");

    let impl_tokens: proc_macro2::TokenStream = impl_src.parse()
        .expect("failed to parse generated impl");

    // Terminal rule — no more tokens, return the built value
    let terminal = if wrap_widget {
        format!("kairos :: Widget :: {name}($expr)")
    } else {
        "$expr".to_string()
    };

    let init = if wrap_widget {
        format!("kairos :: {name} :: new()")
    } else {
        format!("{name} :: new()")
    };

    let has_export = input.attrs.iter().any(|a| a.path().is_ident("export_macro"));
    let macro_export = if wrap_widget || has_export { "#[macro_export]" } else { "" };

    let vec_guard_arm = format!(
        "(@build $expr:expr, children![ vec![$($inner:tt)*], $($rest:tt)*]) => {{ compile_error!(\"Use `children: vec![...]` instead of `children![vec![...]]`. The children! macro is for listing widgets directly, not wrapping a vec.\") }};"
    );

    let children_arm = if wrap_widget {
        format!("(@build $expr:expr, children: $($child:expr),*) => {{ kairos::Widget::{name}($expr.children(vec![$($child),*])) }};")
    } else {
        format!("(@build $expr:expr, children: $($child:expr),*) => {{ $expr.children(vec![$($child),*]) }};")
    };

    let children_macro_arm = format!(
        "(@build $expr:expr, children![$($child:tt)*], $($rest:tt)*) => {{ {macro_name}![@build $expr.children(kairos::children![$($child)*]), $($rest)*] }};"
    );
    
    let children_no_comma_arm = format!(
        "(@build $expr:expr, children![$($child:tt)*]) => {{ compile_error!(\"Missing trailing comma after children![...] inside {macro_name}![...]. Add a ',' after the closing ']'\") }};"
    );
    let missing_comma_arm = format!(
        "(@build $expr:expr, $field:ident : $val:expr) => {{ compile_error!(\"Missing trailing comma in {macro_name}![...]. Add a ',' after the last field value.\") }};"
    );

    let macro_src = format!(
        "{macro_export} macro_rules! {macro_name} {{
            {children_macro_arm}
            {children_no_comma_arm}
            {icon_arms}
            {label_arms}
            {source_arms}
            {nested_arms}
            {push_arms}
            {padding_arms}
            {registry_arms}
            {field_rules}
            {missing_comma_arm}
            {vec_guard_arm}
            {children_arm}
            (@build $expr:expr,) => {{ {terminal} }};
            ($($tokens:tt)*) => {{ {macro_name}![@build {init}, $($tokens)*] }};
            }}"
    );

    let macro_tokens: proc_macro2::TokenStream = macro_src.parse()
        .expect("failed to parse generated macro");

    quote! {
        #clean_input
        #macro_tokens
        #impl_tokens
    }
}