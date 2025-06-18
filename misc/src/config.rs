pub trait ConfigIO {
    fn create_default_config() -> Result<Self, String>
    where
        Self: Sized;

    fn change_config_name(&mut self, name: &str) -> Result<(), String>;
    fn save_parameters(&self) -> Result<(), String>;
    fn load_parameters(&mut self) -> Result<(), String>;
    fn list_existing_configs() -> Result<Vec<String>, String>;
}

enum _ConfigIOError {}
