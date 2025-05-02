use std::env;

pub struct WmInfo {
    pub name: String,
}

impl WmInfo {
    pub fn new() -> Self {
        match env::var("XDG_SESSION_DESKTOP") {
            Ok(result) => WmInfo { name: result },
            Err(_) => WmInfo {
                name: String::from("No WM"),
            },
        }
    }
}
