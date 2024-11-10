//! Backend panel module

use super::Platform;

// mod login;
mod password;
// mod unlock;
mod file_dialog;

/// Backend panel state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BackendPanel {
    /// Whether the panel is open
    pub open: bool,

    // /// The unlocking credential
    // creds: unlock::Credentials,
    password: String,

    file_dialog: file_dialog::FileDialog,
}

impl Default for BackendPanel {
    fn default() -> Self {
        Self {
            open: false,
            // creds: unlock::Credentials::default(),
            password: "default password".to_string(),
            file_dialog: file_dialog::FileDialog::default(),
        }
    }
}

impl BackendPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame, platform: &mut Platform) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Unlock Wallet");
                // Unlock Widget
                ui.add(password::password(&mut self.password));

                // unlock button
                if ui.button("Unlock").clicked() {
                    // unlock the wallet
                }
            });
        });
        ui.separator();

        ui.label("Node");
        self.file_dialog.file_dialog(ui, platform);
        ui.separator();

        ui.label("Peers");
        ui.separator();
    }
}
