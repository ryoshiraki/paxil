pub struct AppConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub gl_version_major: u8,
    pub gl_version_minor: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "paxil".to_string(),
            window_width: 800,
            window_height: 600,
            gl_version_major: 4,
            gl_version_minor: 1,
        }
    }
}
impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
