use chrono::{DateTime, Local};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use lib::Account;
use tauri::{async_runtime::Mutex, Manager, State};

#[tauri::command]
async fn login(
    state: State<'_, Mutex<Account>>,
    username: &str,
    password: &str,
) -> Result<(), String> {
    let mut account = state.lock().await;
    account
        .login(username, password)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_daily_limit(state: State<'_, Mutex<Account>>) -> Result<f64, String> {
    let account = state.lock().await;
    Ok(account.daily())
}

#[tauri::command]
async fn upload(
    state: State<'_, Mutex<Account>>,
    geojson: &str,
    mileage: f64,
    end_time: i64,
) -> Result<(), String> {
    let mut account = state.lock().await;
    let end_time: DateTime<Local> = DateTime::from_timestamp_millis(end_time)
        .ok_or("Invalid timestamp")?
        .with_timezone(&Local);

    account
        .upload_running(geojson, mileage, end_time)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let account = Account::new();
            app.manage(Mutex::new(account));

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![login, get_daily_limit, upload])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
