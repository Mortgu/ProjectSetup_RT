use std::env::consts;
use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent, Wry, Manager};

pub(crate) fn get_menu() -> Menu {
    match consts::OS {
        "macos" => custom_menu_bar(),
        _ => Menu::new(),
    }
}

fn custom_menu_bar() -> Menu {
    let app_menu = Menu::new()
        .add_native_item(MenuItem::About(
            "ProjectSetup".to_string(),
            AboutMetadata::new(),
        ))
        .add_native_item(MenuItem::Separator);

    let file_menu = Menu::new()
        .add_item(CustomMenuItem::new("new_window".to_string(), "New Window").accelerator("CmdOrCtrl+N").disabled())
        .add_item(CustomMenuItem::new("close".to_string(), "Close Window").accelerator("CmdOrCtrl+W"));

    Menu::new()
        .add_submenu(Submenu::new("ProjectSetup", app_menu))
        .add_submenu(Submenu::new("File", file_menu))
}

pub(crate) fn handle_menu_event(event: WindowMenuEvent<Wry>) {
    match event.menu_item_id() {
        "quit" => {
            let app = event.window().app_handle();
            app.exit(0);
        }
        "close" => {
            let window = event.window();

            #[cfg(debug_assertions)]
            if window.is_devtools_open() {
                window.close_devtools();
            } else {
                window.close().unwrap();
            }

            #[cfg(debug_assertions)]
            window.close().unwrap();
        }
        _ => {}
    }
}