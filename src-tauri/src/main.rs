// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use log::{info};
use tauri::async_runtime::Mutex as TauriMutex;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_log::{LogTarget, TimezoneStrategy};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_positioner::{Position, WindowExt};
use time;
// use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

use weevibin::state::{AppState, AppStateMutex, WeeVibinMessage, VibinState, VibinStateMutex};
use weevibin::websocket::{WebSocketManager, WebSocketManagerMutex};

// TODO: Hide when clicking on menu bar away from app window <-- SEEMS OK NOW?
// TODO: Have UI properly show on current virtual desktop rather than always activating the desktop
//  the UI was first opened on

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 3] = [LogTarget::Stdout, LogTarget::Webview, LogTarget::LogDir];

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

/// Called by the UI once it's ready. There's probably a different idiomatic-Tauri way to do this.
#[tauri::command]
async fn on_ui_ready(
    ws_manager: tauri::State<'_, WebSocketManagerMutex>,
    app_state: tauri::State<'_, AppStateMutex>,
    vibin_state: tauri::State<'_, VibinStateMutex>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    app_handle
        .emit_all(&WeeVibinMessage::AppState.to_string(), &*app_state.inner().lock().unwrap())
        .unwrap();
    app_handle
        .emit_all(&WeeVibinMessage::VibinState.to_string(), &*vibin_state.inner().lock().unwrap())
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
            info!("UI requested new Vibin server: {:?}", vibin_server);

            info!("Waiting for WebSocket manager lock");
            let mut manager = ws_manager.inner().lock().await;

            info!("Waiting for WebSocket disconnect");
            manager.stop().await;

            info!("Setting server name: {:?}", vibin_server);
            manager.vibin_host = Some(Box::new(vibin_server));
            manager.start();

            Ok(String::from("OK"))
        }
        Err(e) => {
            let error = format!("Invalid URL: {:?}", e);
            app_handle.emit_all(&WeeVibinMessage::Error.to_string(), &error).unwrap();

            Err(error)
        }
    }
}

fn main() {
    let app_state: AppStateMutex = Arc::new(Mutex::new(AppState::new()));
    let vibin_state: VibinStateMutex = Arc::new(Mutex::new(VibinState::new()));

    let context = tauri::generate_context!();

    // Runtime state
    let app_state_clone = Arc::clone(&app_state);
    let vibin_state_clone = Arc::clone(&vibin_state);

    // Configure the system tray
    let quit = CustomMenuItem::new("quit".to_string(), "Quit WeeVibin");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);

    let system_tray = SystemTray::new().with_menu(system_tray_menu);

    tauri::Builder::default()
        .setup(move |app| {
            info!("Application data directory: {:?}", app.handle().path_resolver().app_data_dir().unwrap());

            let ws_manager_mutex = Arc::new(TauriMutex::new(WebSocketManager::new(
                None,
                Arc::new(Mutex::new(false)),
                app_state_clone,
                vibin_state_clone,
                app.app_handle(),
            )));

            app.manage(ws_manager_mutex);

            // Hide the WeeVibin icon in the macOS dock
            //
            // """ For Windows (from Discord):
            // You're probably looking for the window's set_skip_taskbar
            // (https://docs.rs/tauri/latest/tauri/window/struct.Window.html#method.set_skip_taskbar).
            // The window builder's equivalent:
            // https://docs.rs/tauri/latest/tauri/window/struct.WindowBuilder.html#method.skip_taskbar.
            // """
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            Ok(())
        })
        .manage(app_state)
        .manage(vibin_state)
        .plugin(tauri_plugin_log::Builder::default()
            .targets(LOG_TARGETS)
            .format(|out, message, record| {
                // In macOS, logs are in ~/Library/Logs/<bundle_identifier>/
                let date_format =
                    time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]")
                        .unwrap();

                let colors = ColoredLevelConfig::default();

                out.finish(format_args!(
                    "{} [{}] [{}] {}",
                    TimezoneStrategy::UseUtc.get_now().format(&date_format).unwrap(),
                    colors.color(record.level()),
                    record.target(),
                    message
                ))
            })
            .level(log::LevelFilter::Info)
            .build()
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_positioner::init())
        .enable_macos_default_menu(false)
        .system_tray(system_tray)
        .on_system_tray_event(|app, event|{
            // let window = app.get_window("main").unwrap();
            //
            // #[cfg(target_os = "macos")]
            // apply_vibrancy(
            //     &window,
            //     NSVisualEffectMaterial::HudWindow,
            //     Some(NSVisualEffectState::Active),
            //     Some(7.0)
            // ).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            tauri_plugin_positioner::on_tray_event(app, &event);

            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();

                    // Show the main window. Use TrayCenter as initial window position.
                    let _ = window.move_window(Position::TrayCenter);

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            // Close the UI when clicking outside the window
            tauri::WindowEvent::Focused(is_focused) => {
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![on_ui_ready, set_vibin_server])
        .build(context)
        .expect("Error while building WeeVibin")
        .run(|_app_handle, event| match event {
            // Keep the Rust backend running in the background
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
