// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use tauri::async_runtime::Mutex as TauriMutex;
use tauri::Manager;

use weevibin::state::{AppState, AppStateMutex, Message, VibinState, VibinStateMutex};
use weevibin::websocket::{WebSocketConnection, WebSocketManager, WebSocketManagerMutex};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Called by the UI once it's ready. There's probably a different idiomatic-Tauri way to do this.
#[tauri::command]
async fn on_ui_ready(
    ws_manager: tauri::State<'_, WebSocketManagerMutex>,
    app_state: tauri::State<'_, AppStateMutex>,
    vibin_state: tauri::State<'_, VibinStateMutex>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    println!(">>> {}", &Message::AppState.to_string());
    app_handle
        .emit_all(&Message::AppState.to_string(), &*app_state.inner().lock().unwrap())
        .unwrap();
    app_handle
        .emit_all(&Message::VibinState.to_string(), &*vibin_state.inner().lock().unwrap())
        .unwrap();

    ws_manager.inner().lock().await.start();

    Ok(())
}

/// Set the Vibin WebSocket server URL. e.g. ws://vibin.local:8080/ws
#[tauri::command]
async fn set_vibin_server(
    vibin_server: String,
    ws_manager: tauri::State<'_, WebSocketManagerMutex>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    match url::Url::parse(vibin_server.as_str()) {
        Ok(_) => {
            println!("Setting server: {:?}", vibin_server);

            println!("Command is waiting for WebSocket manager lock");
            let mut manager = ws_manager.inner().lock().await;

            println!("Command is waiting for WebSocket disconnect");
            manager.stop().await;

            println!("Command wait complete");

            *manager.vibin_server = vibin_server;
            manager.start();

            return Ok(String::from("OK"));
        }
        Err(e) => {
            let error = format!("Invalid URL: {:?}", e);
            app_handle.emit_all(&Message::Error.to_string(), &error).unwrap();

            return Err(error);
        }
    }
}

fn main() {
    let app_state: AppStateMutex = Arc::new(Mutex::new(AppState::new()));
    let vibin_state: VibinStateMutex = Arc::new(Mutex::new(VibinState::new()));

    let context = tauri::generate_context!();

    let app_state_clone = Arc::clone(&app_state);
    let vibin_state_clone = Arc::clone(&vibin_state);

    tauri::Builder::default()
        .setup(move |app| {
            let ws_manager_mutex = Arc::new(TauriMutex::new(WebSocketManager::new(
                Arc::new(TauriMutex::new(WebSocketConnection {
                    stop_flag: None,
                    vibin_server: String::from(""),
                })),
                Box::new(String::from("ws://192.168.2.101:8080/ws")),
                Arc::new(Mutex::new(false)),
                app_state_clone,
                vibin_state_clone,
                app.app_handle(),
            )));

            app.manage(ws_manager_mutex);

            Ok(())
        })
        .manage(app_state)
        .manage(vibin_state)
        .invoke_handler(tauri::generate_handler![greet, on_ui_ready, set_vibin_server])
        .run(context)
        .expect("Error while running tauri application");
}
