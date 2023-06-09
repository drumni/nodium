use std::sync::Arc;
use tokio::sync::Mutex;

use log::{debug, error};
use nodium_app::NodiumView;
use nodium_pdk::{NodiumEvent, NodiumNode, NodiumWindow};
use serde_json::{from_str, to_value};
use tauri::{AppHandle, Manager};

// Tauri view
#[derive(Clone)]
pub struct NodiumViewTauri {
    handle: AppHandle,
}

impl NodiumViewTauri {
    pub fn new(handle: AppHandle
    ) -> Self {
        NodiumViewTauri {
            handle: handle,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl NodiumView for NodiumViewTauri {
    fn run(
        &self,
        // async result
    ) -> Result<(), Box<dyn std::error::Error>> {
        // debug!("running tauri view");

      // TODO: connect the event system to the tauri view events from (self.handle.listen_global("event_name", move |event| {});), and self.handle.emit_all("event_name", event_payload)?;

        Ok(())
    }

    fn add_window(&self, window: Box<dyn NodiumWindow>) -> Result<(), Box<dyn std::error::Error>> {
        debug!("adding window: {:?}", window.serialize());
        let event_payload = to_value(window.serialize())?;
        self.handle
            .app_handle()
            .emit_all("add_window", event_payload)?;
        Ok(())
    }

    fn remove_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("removing window: {:?}", window.serialize());
        let event_payload = to_value(window.serialize())?;
        self.handle
            .app_handle()
            .emit_all("remove_window", event_payload)?;
        Ok(())
    }

    fn update_window(
        &self,
        window: Box<dyn NodiumWindow>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("updating window: {:?}", window.serialize());
        let event_payload = to_value(window.serialize())?;
        self.handle
            .app_handle()
            .emit_all("update_window", event_payload)?;
        Ok(())
    }
}
