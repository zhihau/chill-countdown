#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::time::SystemTime;
//use tauri::api::{dialog, menu, shell};

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
  tauri::Builder::default()
.invoke_handler(tauri::generate_handler![get_remaining_time])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
