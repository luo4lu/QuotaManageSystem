#[derive(Clone)]
pub struct ConfigPath {
    pub meta_path: String,
    pub quota_path: String,
}

impl Default for ConfigPath {
    fn default() -> Self {
        Self {
            meta_path: String::from("./meta.json"),
            quota_path: String::from("./quota.json"),
        }
    }
}
