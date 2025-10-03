pub struct Config {
    pub lid_name: String,
}

impl Config {
    /// Get the configuration with default values.
    /// In the future, this could be extended to read from a config file.
    pub fn get() -> Self {
        Config {
            lid_name: "eDP-1".to_string(),
        }
    }
}
