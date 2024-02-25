use anyhow::Result;
use flutter_rust_bridge::frb;

#[frb(sync)]
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[frb(sync)]
pub fn greet_wraped_result(name: String) -> Result<String> {
    Ok(format!("Hello, {name}!"))
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}