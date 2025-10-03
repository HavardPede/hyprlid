use serde::Deserialize;
use std::process::Command;

/// Represents the laptop lid monitor and its state (enabled/disabled).
/// Uses `hyprctl` to check for existence and enable/disable the monitor.
#[derive(Debug, Clone, Deserialize)]
pub struct Lid {
    pub name: String,
    pub enabled: bool,
}

impl Lid {
    pub fn setup(name: &String) -> Self {
        Lid {
            name: name.clone(),
            enabled: Self::exists(&name),
        }
    }

    /// Uses hyprctl to check if a monitor with the given name exists.
    /// We only care to check if it exists. If not, we assume its disabled.
    /// Requires the config to have the correct name for the lid monitor.
    fn exists(name: &String) -> bool {
        let output = Command::new("hyprctl")
            .args(["-j", "monitors"])
            .output()
            .expect("Failed to fetch monitors");

        let monitors: Vec<Monitor> =
            serde_json::from_slice(&output.stdout).expect("failed to parse JSON");

        let lids: Vec<_> = monitors.into_iter().filter(|m| m.has_name(name)).collect();

        lids.len() == 1
    }

    /// Disable the monitor by name using hyprctl.
    /// Read the docs here: https://wiki.hypr.land/Configuring/Monitors/#disabling-a-monitor
    pub fn disable(&mut self) {
        self.enabled = false;
        self.set_availability("disable");
    }

    /// Enable the monitor by name using hyprctl.
    /// Read the docs here: https://wiki.hypr.land/Configuring/Monitors/#disabling-a-monitor
    pub fn enable(&mut self) {
        self.enabled = true;
        self.set_availability("enable");
    }

    fn set_availability(&self, action: &str) {
        let monitor_spec = format!("{}, {}", self.name, action);
        Command::new("hyprctl")
            .args(["keyword", "monitor", &monitor_spec])
            .output()
            .expect("Failed to set monitor availability");
    }
}

/// Simple struct to deserialize monitor info from hyprctl.
/// More fields are available, but we only care about the name.
#[derive(Debug, Clone, Deserialize)]
struct Monitor {
    name: String,
}

impl Monitor {
    fn has_name(&self, name: &str) -> bool {
        self.name == name
    }
}
