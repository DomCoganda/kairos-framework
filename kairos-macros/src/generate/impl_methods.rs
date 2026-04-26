use syn::Fields;

pub fn build(field_names: &[String], field_types: &[String], fields: &Fields, macro_name: &str) -> String {

    let nested_field_names: std::collections::HashSet<String> = fields.iter()
        .filter(|f| f.attrs.iter().any(|a| a.path().is_ident("nested")))
        .filter_map(|f| f.ident.as_ref())
        .map(|f| f.to_string())
        .collect();

    let widget_field_names: std::collections::HashSet<String> = fields.iter()
        .filter(|f| f.attrs.iter().any(|a| a.path().is_ident("widget")))
        .filter_map(|f| f.ident.as_ref())
        .map(|f| f.to_string())
        .collect();

    let push_field_names: std::collections::HashSet<String> = fields.iter()
        .filter(|f| f.attrs.iter().any(|a| a.path().is_ident("push")))
        .filter_map(|f| f.ident.as_ref())
        .map(|f| f.to_string())
        .collect();
    
    let impl_methods: String = field_names.iter().zip(field_types.iter())
        .map(|(f, t)| {
            if widget_field_names.contains(f) {
                format!("pub fn {f}(mut self, value: impl Fn(&Palette) -> Widget + Send + 'static) -> Self {{ self.{f} = Box::new(value); self }}")
            } else if t.contains("dyn") {
                format!("pub fn {f}(mut self, value: {t}) -> Self {{ self.{f} = value; self }}")
            } else if t.starts_with("Option <") && nested_field_names.contains(f) {
                let inner = t.trim_start_matches("Option <").trim_end_matches('>').trim();
                format!("pub fn {f}(mut self, value: Option<{inner}>) -> Self {{ self.{f} = value; self }}")
            } else if t.starts_with("Option <") {
                let inner = t.trim_start_matches("Option <").trim_end_matches('>').trim();
                format!("pub fn {f}(mut self, value: impl Into<{inner}>) -> Self {{ self.{f} = Some(value.into()); self }}")
            } else if t.contains("Arc") && t.contains("dyn Fn") {
                format!("pub fn {f}(mut self, value: impl Fn(&Palette) -> Widget + Send + Sync + 'static) -> Self {{ self.{f} = std::sync::Arc::new(value); self }}")
            } else if push_field_names.contains(f) {
                let inner_type = t
                    .trim_start_matches("Vec <")
                    .trim_end_matches('>')
                    .trim();
                format!("pub fn push_{f}(mut self, value: {inner_type}) -> Self {{ self.{f}.push(value); self }}")
            } else {
                format!("pub fn {f}(mut self, value: impl Into<{t}>) -> Self {{ self.{f} = value.into(); self }}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    impl_methods
}