use std::path::PathBuf;
use std::process::Command;
use dirs;

pub fn sh(command:&str) -> String {
    let take = Command::new("bash")
        .args(["-c", command])
        .output()
        .expect("An error has occured while trying to get object");

    String::from_utf8_lossy(&take.stdout).trim().to_string()
}

pub fn get_config_dir() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join(".config").join("jewfetch")
    } else {
        PathBuf::new()
    }
}
