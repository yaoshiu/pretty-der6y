/*
    Pretty Der6y - A third-party running data upload client.
    Copyright (C) 2024  Fay Ash

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use lib::{
    chrono::{DateTime, Local},
    Account,
};
use specta_typescript::{formatter, BigIntExportBehavior, Typescript};
use tauri::{async_runtime::Mutex, Manager, State};
use tauri_specta::{collect_commands, Builder};

#[tauri::command]
#[specta::specta]
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
#[specta::specta]
async fn get_daily_limit(state: State<'_, Mutex<Account>>) -> Result<f64, String> {
    let account = state.lock().await;
    Ok(account.daily())
}

#[tauri::command]
#[specta::specta]
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
        .upload_running(geojson, mileage, &end_time)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _builder =
        Builder::<tauri::Wry>::new().commands(collect_commands![login, get_daily_limit, upload,]);

    #[cfg(debug_assertions)] // only export typescript bindings in debug mode
    _builder
        .export(
            Typescript::default()
                .bigint(BigIntExportBehavior::Number)
                .formatter(formatter::biome),
            "../src/helpers/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;

            let account = Account::new();
            app.manage(Mutex::new(account));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![login, get_daily_limit, upload])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
