//! Platform code, specific to the native platform.
//!
//! For example, a native node will only be available here. Whereas the browser needs to connect
//! to a remote node, which is handled in the `web` module.

pub(crate) fn show(this: &mut super::TemplateApp, ui: &mut egui::Ui) {
    // Show "Launching Local node" status
    ui.horizontal(|ui| {
        ui.label("Launching Local node: ");
        let text_edit = egui::TextEdit::singleline(&mut this.label).margin(egui::vec2(10.0, 5.0));
        ui.add(text_edit);
    });
}
