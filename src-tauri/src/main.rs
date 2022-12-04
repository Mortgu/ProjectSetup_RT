#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api::path,
    http::{ResponseBuilder, Uri},
    Manager, RunEvent,
};

use tokio::task::block_in_place;

mod macos;
mod menu;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app = app.handle();

            app.windows().iter().for_each(|(_, window)| {
              //window.hide().unwrap();

              #[cfg(target_os = "macos")] {
                  use macos::*;

                  let window = window.ns_window().unwrap();
                 // set_titlebar_style(window, true, true);
                 // blur_window_background(window);
              }
          });
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
