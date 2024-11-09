//! Backend panel module

mod login;
mod password;
mod unlock;

/// Backend panel state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BackendPanel {
    /// Whether the panel is open
    pub open: bool,

    /// The unlocking credential
    creds: unlock::Credentials,

    password: String,
}

impl Default for BackendPanel {
    fn default() -> Self {
        Self {
            open: false,
            creds: unlock::Credentials::default(),
            password: "default password".to_string(),
        }
    }
}

impl BackendPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
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
        ui.separator();

        ui.label("Peers");
        ui.separator();
    }
}
