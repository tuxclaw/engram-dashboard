use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::Duration;

use tauri::Manager;

const APP_URL: &str = "http://localhost:3460";
const STATUS_URL: &str = "http://localhost:3460/api/status";
const SERVER_PATH: &str = "/home/tux/.openclaw/workspace/engram/dashboard/server.py";

struct BackendProcess {
    child: Mutex<Option<Child>>,
}

impl BackendProcess {
    fn spawn() -> Result<Self, String> {
        let child = Command::new("python3")
            .arg(SERVER_PATH)
            .env("ENGRAM_DASHBOARD_PORT", "3460")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|err| format!("Failed to start backend: {err}"))?;

        Ok(Self {
            child: Mutex::new(Some(child)),
        })
    }
}

impl Drop for BackendProcess {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.child.lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.kill();
            }
        }
    }
}

fn wait_for_backend() -> Result<(), String> {
    tauri::async_runtime::block_on(async {
        let client = reqwest::Client::new();
        let mut last_err = None;

        for _ in 0..60 {
            match client.get(STATUS_URL).send().await {
                Ok(response) if response.status().is_success() => return Ok(()),
                Ok(response) => {
                    last_err = Some(format!("Status check returned {}", response.status()));
                }
                Err(err) => {
                    last_err = Some(err.to_string());
                }
            }

            tokio::time::sleep(Duration::from_millis(250)).await;
        }

        Err(format!(
            "Backend did not become ready in time: {}",
            last_err.unwrap_or_else(|| "unknown error".to_string())
        ))
    })
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let backend = BackendProcess::spawn()?;
            app.manage(backend);

            wait_for_backend()?;

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.eval(&format!(
                    "window.location.replace('{}')",
                    APP_URL
                ));
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
