//! Platform Module

/// Native Platform Module
#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
use native as platform;

/// Web Platform Module
#[cfg(target_arch = "wasm32")]
use multinode_web::platform;

pub(crate) use platform::Platform;
