//! Platform code, specific to the native platform.
//!
//! For example, a native node will only be available here. Whereas the browser needs to connect
//! to a remote node, which is handled in the `web` module.

use std::io::{BufRead as _, BufReader};
use std::process::Child;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;

/// Track whether the Context has been set
pub(crate) struct ContextSet {
    /// Whether the Context has been set
    pub(crate) set: bool,

    /// The Context
    pub(crate) ctx: egui::Context,
}

pub(crate) struct Platform {
    // This is where you would put platform-specific fields
    server_process: Option<Child>,

    inbox: std::sync::mpsc::Receiver<String>,

    log: Arc<Mutex<Vec<String>>>,

    /// Clone of the [egui::Context] so that the platform can trigger repaints
    ctx: Arc<Mutex<ContextSet>>,
}

impl Default for Platform {
    fn default() -> Self {
        let log = Arc::new(Mutex::new(Vec::new()));
        let ctx: Arc<Mutex<ContextSet>> = Arc::new(Mutex::new(ContextSet {
            set: false,
            ctx: egui::Context::default(),
        }));

        #[cfg(debug_assertions)]
        let program = {
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

        tracing::info!("server_bin_path: {:?}", program);

        // Create a communication channel between parent and child tasks
        let (tx, mut rx): (Sender<String>, Receiver<String>) = channel(100);

        // Set up a sync channel to control the child process, mainly to kill() it when eframe exits
        let (tx_control, rx_control) = std::sync::mpsc::channel();

        let (outbox, inbox) = std::sync::mpsc::channel();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Unable to create Runtime");

        // Enter the runtime so that `tokio::spawn` is available immediately.
        let _enter = rt.enter();

        let log_clone = log.clone();
        let ctx_clone = ctx.clone();

        // Execute the runtime in its own thread.
        // The future doesn't have to do anything. In this example, it just sleeps forever.
        std::thread::spawn(move || {
            rt.block_on(async {
                // Receive messages from the async task and update the GUI
                let message_task = tokio::task::spawn(async move {
                    while let Some(message) = rx.recv().await {
                        tracing::info!("[child_msg] {}", message);
                        // push onto log
                        log_clone.lock().unwrap().push(message);
                        // use ctx to repaint, if Set
                        if ctx_clone.lock().unwrap().set {
                            ctx_clone.lock().unwrap().ctx.request_repaint();
                        } else {
                            tracing::warn!("No ctx to repaint");
                        }
                    }
                });

                // Spawn the child process in a separate Tokio task
                let child_task = task::spawn(async move {
                    tracing::info!("Spawning child process");
                    let mut child = Command::new(program)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to start child process");

                    // Read stdout and stderr from the child process
                    let stdout = child.stdout.take().expect("Failed to capture stdout");
                    let stderr = child.stderr.take().expect("Failed to capture stderr");

                    // Send output from the child process to the parent task
                    let tx_clone = tx.clone();
                    let stdout_task = task::spawn_blocking(move || {
                        for line in BufReader::new(stdout).lines() {
                            // clean up the text, as it shows:
                            //[2m2024-11-06T21:25:08.224398Z[0m [32m INFO[0m [2mpeerpiper_core::libp2p::api[0m[2m:[0m
                            // get rid of the [ garbage that doesn't display well outside the terminal
                            // remove anything that starts with ^[[ and ends with m
                            let text = line.unwrap().to_string();
                            let line = text
                                .split("\x1b")
                                .map(|s| {
                                    if s.starts_with("[") {
                                        let mut split = s.split("m");
                                        split.next();
                                        split.collect::<String>()
                                    } else {
                                        s.to_string()
                                    }
                                })
                                .collect::<String>();

                            tx_clone
                                .blocking_send(line)
                                .expect("Failed to send message to parent task");
                        }
                    });

                    let tx_clone = tx.clone();
                    let stderr_task = task::spawn_blocking(move || {
                        for line in BufReader::new(stderr).lines() {
                            // make sure all the text is preserved, ASCII and UTF-8, everything,
                            // emojiis and all
                            let all_text = line.unwrap().chars().collect::<Vec<char>>();
                            let mut text = String::new();
                            for c in all_text {
                                text.push(c);
                            }

                            let line = text;
                            tx_clone
                                .blocking_send(format!("Stderr: {}", line))
                                .expect("Failed to send message to parent task");
                        }
                    });

                    tracing::info!("Child process spawned successfully");

                    // Send the child process handle to the parent task
                    if let Err(e) = tx_control.send(child) {
                        tracing::error!(
                            "Failed to send child process handle to parent task: {:?}",
                            e
                        );
                    }

                    // Wait for the stdout and stderr tasks to complete
                    if let Err(e) = stdout_task.await {
                        tracing::error!("Stdout task panicked: {:?}", e);
                    }

                    if let Err(e) = stderr_task.await {
                        tracing::error!("Stderr task panicked: {:?}", e);
                    }
                });

                // Wait for either the child task to finish or the message task to complete
                tokio::select! {
                    child_result = child_task => {
                        if let Err(e) = child_result {
                            tracing::error!("Child task panicked: {:?}", e);
                        }
                    }
                    _ = message_task => {
                        tracing::info!("Message processing task completed");
                    }
                }
            });
        });

        // Wait for the child process to start
        let server_process = rx_control
            .recv()
            .expect("Failed to receive child process handle");

        Self {
            server_process: Some(server_process),
            inbox,
            log,
            ctx,
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
    /// Returns whether the ctx is set or not
    pub(crate) fn egui_ctx(&self) -> bool {
        self.ctx.lock().unwrap().set
    }

    /// Stes the ctx
    pub(crate) fn set_egui_ctx(&mut self, ctx: egui::Context) {
        self.ctx.lock().unwrap().ctx = ctx;
        self.ctx.lock().unwrap().set = true;
    }

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

    /// Platform specific UI to show
    pub(crate) fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // label for Log
        ui.separator();
        ui.label("Log:");

        // SCROLLABLE SECTION for the log
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                for line in self.log.lock().unwrap().iter().rev() {
                    ui.label(line);
                }
            });
        });

        ui.separator();
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
