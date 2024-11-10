mod backend_panel;
pub mod error;
mod platform;
mod style;

pub use error::Error;

use backend_panel::BackendPanel;
use eframe::glow::Context;
pub(crate) use platform::Platform;
use style::is_mobile;

const IS_WEB: bool = cfg!(target_arch = "wasm32");

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct State {
    backend_panel: BackendPanel,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    /// State
    state: State,

    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    // Platform specific fields
    #[serde(skip)]
    /// Platform  specific handlers for native and web     
    platform: Platform,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            platform: Default::default(),
            label: "/dnsaddr/peerpiper.io/".to_owned(),
            value: 2.7,
            state: Default::default(),
        }
    }
}

/// APP_KEY constant, concat of eframe::APP_KEY and crate name
const APP_KEY: &str = concat!("eframe-app-", env!("CARGO_PKG_NAME"));

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // crate::style::fonts(&cc.egui_ctx);

        eprintln!("app_key: {}", APP_KEY);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            if let Some(app) = eframe::get_value(storage, APP_KEY) {
                tracing::info!("💾 💾 💾 Loaded app state from disk");
                return app;
            }
        }

        tracing::info!("🆕 🆕 🆕 No app state found on disk");

        Self {
            platform: Default::default(),
            label: "/dnsaddr/peerpiper.io/".to_owned(),
            value: 2.7,
            state: Default::default(),
        }
    }

    /// Contents of the Top Bar
    fn bar_contents(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::widgets::global_theme_preference_switch(ui);

        if is_mobile(ui.ctx()) {
            ui.menu_button("💻 Backend", |ui| {
                ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
            });
        } else {
            ui.toggle_value(&mut self.state.backend_panel.open, "💻 Menu");
        }

        egui::menu::bar(ui, |ui| {
            if !IS_WEB {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            }
        });
    }

    fn backend_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // The backend-panel can be toggled on/off.
        // We show a little animation when the user switches it.
        let is_open =
            self.state.backend_panel.open || ctx.memory(|mem| mem.everything_is_visible());

        egui::SidePanel::left("backend_panel")
            .resizable(true)
            .show_animated(ctx, is_open, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("💻 Backend");
                });

                ui.separator();
                self.state.backend_panel.ui(ui, frame, &self.platform);
            });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        tracing::info!("💾 💾 💾 Saving app state to disk, key: {}", APP_KEY);
        eframe::set_value(storage, APP_KEY, self);
    }

    /// Kill server on exit
    fn on_exit(&mut self, _gl: Option<&Context>) {
        // drop the platform to Kill the server process
        self.platform.close();
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // pass the ctx to the platform
        if !self.platform.egui_ctx() {
            self.platform.set_egui_ctx(ctx.clone());
        }

        // set the style
        style::style(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ctx, ui, frame);
            });
        });

        if !is_mobile(ctx) {
            self.backend_panel(ctx, frame);
        }

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                ui.add(egui::github_link_file!(
                    "https://github.com/PeerPiper/egui-multinode/blob/main/",
                    format!("🖹 Rust Source Code")
                ));
                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.vertical(|ui| {
                self.platform.show(ctx, ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
