use std::env;
pub mod linux_platform;
pub mod windows_platform;
use crate::linux_platform::linux_lock_unlock;
use crate::windows_platform::windows_lock_unlock;
#[tokio::main]
async fn main() {
    match env::consts::OS {
        "windows" => windows_lock_unlock().await,
        "linux" => linux_lock_unlock().await,
        "macos" => default(),
        "android" => default(),
        "ios" => default(),
        _ => default(),
    }
}

fn default() {
    println!("Default");
}
