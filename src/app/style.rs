// use std::collections::BTreeMap;

// use egui::{FontId, TextStyle};

/// Function to set the style
pub(crate) fn style(ctx: &egui::Context) {
    ctx.style_mut(|style| {
        // Increase padding for all widgets
        // style.spacing.item_spacing = egui::vec2(10.0, 10.0);

        // Increase padding specifically for buttons
        style.spacing.button_padding = egui::vec2(10.0, 5.0);

        // style.text_styles = BTreeMap::from([
        //     (
        //         TextStyle::Heading,
        //         FontId::new(24.0, egui::FontFamily::Proportional),
        //     ),
        //     (
        //         TextStyle::Body,
        //         FontId::new(18.0, egui::FontFamily::Proportional),
        //     ),
        //     (
        //         TextStyle::Monospace,
        //         FontId::new(16.0, egui::FontFamily::Monospace),
        //     ),
        //     (
        //         TextStyle::Button,
        //         FontId::new(16.0, egui::FontFamily::Proportional),
        //     ),
        //     (
        //         TextStyle::Small,
        //         FontId::new(12.0, egui::FontFamily::Proportional),
        //     ),
        // ]);
    });
}

/// Detect narrow screens.
///
/// This is used to show a simpler UI on mobile devices, especially for the web
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}
