//! Backend panel module

use super::platform::Platform;

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
    pub fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame, platform: &Platform) {
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
        let platform_clone = platform.clone();
        let on_load_callback = move |name, bytes| {
            platform_clone.load_plugin(name, bytes);
        };
        if let Err(e) = self.file_dialog.file_dialog(ui, on_load_callback) {
            tracing::error!("Failed to open file dialog: {:?}", e);
        }
        ui.separator();

        ui.label("Peers");
        ui.separator();
    }
}
