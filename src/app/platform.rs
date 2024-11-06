//! Platform Module

use super::*;

/// Native Platform Module
#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
use native as platform;

/// Web Platform Module
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
use web as platform;

pub(crate) use platform::Platform;

/// The traits any chosen Platform must implement
pub enum PlatformEnum {
    #[cfg(not(target_arch = "wasm32"))]
    Native(platform::Platform),
    #[cfg(target_arch = "wasm32")]
    Web(platform::Platform),
}

impl Default for PlatformEnum {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        return Self::Native(Default::default());
        #[cfg(target_arch = "wasm32")]
        return Self::Web(Default::default());
    }
}

impl PlatformEnum {
    pub fn close(&mut self) {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::Native(platform) => platform.close(),
            #[cfg(target_arch = "wasm32")]
            Self::Web(platform) => platform.close(),
        }
    }
}

pub(crate) fn show(this: &mut super::TemplateApp, ui: &mut egui::Ui) {
    platform::show(this, ui);
}
