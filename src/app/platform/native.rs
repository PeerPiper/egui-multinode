//! Platform code, specific to the native platform.
//!
//! For example, a native node will only be available here. Whereas the browser needs to connect
//! to a remote node, which is handled in the `web` module.

use std::process::{Child, Command};

pub(crate) struct Platform {
    // This is where you would put platform-specific fields
    server_process: Option<Child>,
}

impl Default for Platform {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        let server_bin_path = {
            let path = std::env::current_dir()
                .unwrap()
                .join("../peerpiper/target/debug/peerpiper-server");
            // check to ensure the server binary exists, otherwise return bin/peerpiper-server
            if path.exists() {
                path
            } else {
                std::env::current_dir()
                    .unwrap()
                    .join("bin/peerpiper-server")
            }
        };

        #[cfg(not(debug_assertions))]
        let server_bin_path = std::env::current_dir()
            .unwrap()
            .join("bin/peerpiper-server");

        println!("server_bin_path: {:?}", server_bin_path);

        let server_process = Command::new(server_bin_path)
            .spawn()
            .expect("Failed to start server");

        Self {
            server_process: Some(server_process),
        }
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        // Kill the server process
        if let Some(mut server_process) = self.server_process.take() {
            tracing::info!("Killing server process on drop");
            server_process.kill().expect("Failed to kill server");
        }
    }
}

impl Platform {
    // This is where you would put platform-specific methods
    pub(crate) fn close(&mut self) {
        // Kill the server process
        if let Some(mut server_process) = self.server_process.take() {
            tracing::info!("Killing server process on close");
            match server_process.kill() {
                Ok(_) => {
                    tracing::info!("Server process killed successfully");
                    match server_process.wait() {
                        Ok(status) => {
                            tracing::info!("Server process exited with status: {:?}", status);
                        }
                        Err(e) => {
                            tracing::error!("Failed to wait for server process: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to kill server process: {:?}", e);
                    if let Err(e) = server_process.try_wait() {
                        tracing::error!("Failed to wait for server process: {:?}", e);
                    }
                }
            }
        }
    }
}

pub(crate) fn show(this: &mut super::TemplateApp, ui: &mut egui::Ui) {
    // Show "Launching Local node" status
    ui.horizontal(|ui| {
        ui.label("Launching Local node: ");
        let text_edit = egui::TextEdit::singleline(&mut this.label).margin(egui::vec2(10.0, 5.0));
        ui.add(text_edit);
    });
}
