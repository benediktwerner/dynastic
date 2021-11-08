#![cfg_attr(
    all(not(debug_assertions), target_os = "windows", feature = "tauri"),
    windows_subsystem = "windows"
)]

fn main() {
    #[cfg(feature = "tauri")]
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
