use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Haptics;
#[cfg(mobile)]
use mobile::Haptics;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the haptics APIs.
pub trait HapticsExt<R: Runtime> {
  fn haptics(&self) -> &Haptics<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HapticsExt<R> for T {
  fn haptics(&self) -> &Haptics<R> {
    self.state::<Haptics<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("haptics")
    .invoke_handler(tauri::generate_handler![commands::execute])
    .setup(|app, api| {
      #[cfg(mobile)]
      let haptics = mobile::init(app, api)?;
      #[cfg(desktop)]
      let haptics = desktop::init(app, api)?;
      app.manage(haptics);

      // manage state so it is accessible by the commands
      app.manage(MyState::default());
      Ok(())
    })
    .build()
}
