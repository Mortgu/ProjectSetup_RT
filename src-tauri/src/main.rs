#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod menu;
mod window;
use window::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_transparent_titlebar(true);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .menu(menu::get_menu())
        .on_menu_event(menu::handle_menu_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command(invoke_message: String) -> String {
    let message = format!("{}", invoke_message);
    return message;
}
