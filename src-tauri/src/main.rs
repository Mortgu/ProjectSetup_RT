#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Menu;
mod menu;

fn main() {
  let menu = Menu::new();
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![my_custom_command])
  .menu(menu::get_menu())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command(invoke_message: String) -> String {
  let message = format!("{}", invoke_message);
  return message;
}