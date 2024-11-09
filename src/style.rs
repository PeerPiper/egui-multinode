use eframe::{
    egui,
    epaint::text::{FontInsert, InsertFontFamily},
};

/**/
// ctx.style_mut(|style| {
//     style.text_styles = BTreeMap::from([
//         (TextStyle::Heading, FontId::new(24.0, egui::FontFamily::Proportional)),
//         (TextStyle::Body, FontId::new(18.0, egui::FontFamily::Proportional)),
//         (TextStyle::Monospace, FontId::new(16.0, egui::FontFamily::Monospace)),
//         (TextStyle::Button, FontId::new(16.0, egui::FontFamily::Proportional)),
//         (TextStyle::Small, FontId::new(12.0, egui::FontFamily::Proportional)),
//     ]);
// });

pub(crate) fn fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let font = include_bytes!("./app/fonts/HackNerdFont-Regular.ttf");

    tracing::info!("Adding font to egui context {:?}", font.len());

    fonts.font_data.insert(
        "hack_nerd_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(font)),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "hack_nerd_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("hack_nerd_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
    // ctx.add_font(FontInsert::new(
    //     "hack_nerd_font",
    //     egui::FontData::from_static(font),
    //     vec![
    //         InsertFontFamily {
    //             family: egui::FontFamily::Proportional,
    //             priority: egui::epaint::text::FontPriority::Highest,
    //         },
    //         InsertFontFamily {
    //             family: egui::FontFamily::Monospace,
    //             priority: egui::epaint::text::FontPriority::Lowest,
    //         },
    //     ],
    // ));
}
