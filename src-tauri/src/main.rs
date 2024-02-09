#![windows_subsystem = "windows"]

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .run(context)
        .expect("error while running tauri application");
}
