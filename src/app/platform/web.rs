use super::*;

#[derive(Default)]
pub(crate) struct Platform;

impl Platform {
    pub(crate) fn close(&mut self) {}
}

pub(crate) fn show(this: &mut super::TemplateApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Connect to multinode: ");
        // ui.text_edit_singleline(&mut self.label);
        let text_edit = egui::TextEdit::singleline(&mut this.label).margin(egui::vec2(10.0, 5.0));
        ui.add(text_edit);
    });
}
