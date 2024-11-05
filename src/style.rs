ctx.style_mut(|style| {
    style.text_styles = BTreeMap::from([
        (TextStyle::Heading, FontId::new(24.0, egui::FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(18.0, egui::FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(16.0, egui::FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(16.0, egui::FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(12.0, egui::FontFamily::Proportional)),
    ]);
});
