//! Windows registry read helpers for Nvidia Omniverse View.

pub const APP_KEY: &str = "Software\\Nvidia\\Nvidia Omniverse View";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Nvidia Omniverse View";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Nvidia Omniverse View") },
        RegValue { name: "Version",     value: String::from("v1.2.8") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Nvidia Omniverse View") },
        RegValue { name: "Publisher",       value: String::from("Nvidia") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Nvidia Omniverse View\\uninstall.exe\"") },
    ]
}
