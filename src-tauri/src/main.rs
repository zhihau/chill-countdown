#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::time::SystemTime;
//use tauri::api::{dialog, menu, shell};

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;

#[tauri::command]
fn get_remaining_time(endTime: u64) -> String {
println!("Message from Rust: {}", endTime);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let remaining_time = endTime - now;
    format!(
        "{} days, {} hours, {} minutes, {} seconds",
        remaining_time / (24 * 60 * 60),
        (remaining_time / (60 * 60)) % 24,
        (remaining_time / 60) % 60,
        remaining_time % 60
    )
}

fn main() {
// here `"exit".to_string()` defines the menu item id, and the second parameter is the menu item label.
let exit = CustomMenuItem::new("exit".to_string(), "Exit");
let hide = CustomMenuItem::new("hide".to_string(), "Hide");
let show= CustomMenuItem::new("show".to_string(), "Show");
let about = CustomMenuItem::new("about".to_string(), "About");
let tray_menu = SystemTrayMenu::new()
  .add_item(show)
  .add_item(hide)
 .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(about)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(exit);
let system_tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
.on_window_event(|event| match event.event() {
//click close button to hide window
// thanks https://github.com/tauri-apps/tauri/discussions/2684
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        }) 
.system_tray(system_tray)
.on_system_tray_event(|app, event| match event {
SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a left click");
      }
      SystemTrayEvent::RightClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a right click");
      }
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a double click");
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "exit" => {
            std::process::exit(0);
          }
          "hide" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
          }
          "about" => {
            let window = app.get_window("main").unwrap();
window.eval("window.location.replace('about.html')");
          }
          "show" => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
          }
          _ => {}
        }
      }
      _ => {}
    })
.invoke_handler(tauri::generate_handler![get_remaining_time])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
