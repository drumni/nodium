// use nodium_app::NodiumApp;
// use nodium_taui::TauriRenderer;
// use nodium_events::EventBus;
// use nodium_plugins::Plugins;
// use tokio::runtime::Runtime;
// use env_logger::Builder;
// use log::{debug, info, LevelFilter};

// fn main() {
//     Builder::new()
//         .filter(None, LevelFilter::Debug) // Change this to the desired log level
//         .init();
//     debug!("Logger initialized");
//     // welcome
//     println!("Welcome to Nodium!\n");

//     info!("Creating runtime");
//     let rt = Runtime::new().unwrap();
//     debug!("Runtime created");
//     rt.block_on(async {
//         info!("Creating event bus");
//         let event_bus = EventBus::new();
//         debug!("Event bus created");

//         info!("Creating plugins");
//         let plugins = Plugins::new(event_bus.clone()).await;
//         debug!("Plugins created");

//         info!("Registering event handlers");
//         plugins.lock().await.listen().await;
//         debug!("Event handlers registered");

//         plugins.lock().await.reload().await;

//         info!("NodiumApp starting");

//         let renderer = TauriRenderer::new();

//         let app = NodiumApp::new(event_bus, Box::new(renderer));
//         debug!("NodiumApp created");

//     });
// }


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}