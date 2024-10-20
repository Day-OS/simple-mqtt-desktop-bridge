use std::sync::{Arc, Mutex};

use regex::Regex;
use rumqttc::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Shutdown {
    time: String,
}

pub fn on_sleep_request(_: Arc<Mutex<Client>>, payload: String) {
    let sleep_json: Result<Shutdown, serde_json::Error> =
        serde_json::from_str::<Shutdown>(&payload);
    if let Err(e) = sleep_json {
        log::info!("Wrong input when trying to send sleep command: {e}");
        return;
    }
    let sleep_command: Shutdown = sleep_json.unwrap();

    let reg = Regex::new(r"^\s*(now|\+?[0-9]+(:[0-5][0-9])?([smhdSMHD])?)\s*$").unwrap();
    if !reg.is_match(&sleep_command.time) {
        log::info!("Invalid time informed to command");
        return;
    }

    let command = std::process::Command::new("shutdown")
        .arg(sleep_command.time.trim())
        .output();

    match command {
        Ok(output) => {
            let mut output_text: String = "".to_owned();
            if !output.clone().stdout.is_empty() {
                output_text = String::from_utf8(output.clone().stdout).unwrap();
                log::info! {"{}", output_text};
            }

            if !output.stderr.is_empty() {
                output_text = String::from_utf8(output.stderr).unwrap();
                log::error! {"{}", output_text};
            }

            notify_rust::Notification::new()
                .summary("Shutdown Command")
                .body(&output_text)
                .show()
                .unwrap();
        }
        Err(e) => {
            notify_rust::Notification::new()
                .summary("Shutdown Command error")
                .body(&e.to_string())
                .show()
                .unwrap();
            log::info!("SLEEP COMMAND ERROR: {e}");
        }
    }
}
