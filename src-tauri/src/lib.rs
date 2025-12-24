// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
// use window_vibrancy::apply_acrylic;
use window_vibrancy::apply_mica;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            {
                //TODO: Add a fallback for unsupported platforms
                let _ = apply_mica(&window, None).expect("Unsupported Platform");
                // apply_acrylic(&window, Some((18, 18, 18, 125)))
                //     .expect("Unsupported platform! 'apply_acrylic' is failed.");
                // todo!("Add option for mica on unsupported platform");
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_mica_effect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn set_mica_effect(window: tauri::Window) {
    #[cfg(target_os = "windows")]
    let _ = apply_mica(&window, None);
}
<<<<<<< HEAD

#[tauri::command]
fn check_first_run() -> bool {
    // We define an internal helper to keep the '?' syntax for easy error handling
    fn internal_check() -> std::io::Result<bool> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let software = hkcu.open_subkey("Software")?;

        // Open or Create the "open_vault" key
        let (vault_key, _disp) = software.create_subkey("open_vault")?;

        // Try to get the value
        let is_installed: Result<u32, _> = vault_key.get_value("Installed");

        match is_installed {
            Ok(_) => {
                // Value exists -> Not first run
                Ok(false)
            }
            Err(_) => {
                // Value missing -> First run
                // Create the value now so next time returns false
                // vault_key.set_value("Installed", &1u32)?;
                // We dont need to set any
                Ok(true)
            }
        }
    }

    // Run the check. If ANY error happens (permissions, etc),
    // log it to the console and default to true (First Run).
    match internal_check() {
        Ok(is_first) => is_first,
        Err(e) => {
            println!("Registry error (defaulting to First Run): {}", e);
            true
        }
    }
}
=======
>>>>>>> parent of 88b5759 ([v0.3.0]Added a first run detector)
