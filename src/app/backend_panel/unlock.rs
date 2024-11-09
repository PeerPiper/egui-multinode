//! A egui Widget for unlocking a Wallet with a Username and Password

use std::ops::{Deref, DerefMut};

use seed_keeper_core::credentials::MinString;
use serde::{Deserialize, Serialize};

/// The unlock Widget
pub fn unlock(creds: &mut Credentials) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| unlock_ui(ui, creds)
}

/// The Credentials
#[derive(Serialize, Deserialize)]
pub struct Credentials(seed_keeper_core::credentials::Credentials);

impl Default for Credentials {
    fn default() -> Self {
        Self(seed_keeper_core::credentials::Credentials {
            encrypted_seed: None,
            username: MinString::<8>::default(),
            password: MinString::<8>::default(),
        })
    }
}

impl Deref for Credentials {
    type Target = seed_keeper_core::credentials::Credentials;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Credentials {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// This function shows 3 lines of UI:
///
/// 1. An encrypted seed field
/// 2. A username field
/// 3. A password field
fn unlock_ui(ui: &mut egui::Ui, creds: &mut Credentials) -> egui::Response {
    // This widget has its own state — show or hide password characters (`show_plaintext`).
    // In this case we use a simple `bool`, but you can also declare your own type.
    // It must implement at least `Clone` and be `'static`.
    // If you use the `persistence` feature, it also must implement `serde::{Deserialize, Serialize}`.

    // Generate an id for the state
    let state_id = ui.id().with("show_plaintext");

    // Get state for this widget.
    // You should get state by value, not by reference to avoid borrowing of [`Memory`].
    let mut show_plaintext = ui.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

    // Process ui, change a local copy of the state
    // We want TextEdit to fill entire space, and have button after that, so in that case we can
    // change direction to right_to_left.
    let result = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        // Toggle the `show_plaintext` bool with a button:
        let response = ui
            .add(egui::SelectableLabel::new(show_plaintext, "👁"))
            .on_hover_text("Show/hide password");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }

        // Show the password field:
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::singleline(&mut creds.password.to_string()).password(!show_plaintext),
        );
    });

    // Store the (possibly changed) state:
    ui.data_mut(|d| d.insert_temp(state_id, show_plaintext));

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, …) and maybe show a tooltip:
    result.response
}
