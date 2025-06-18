pub trait ConfigIO {
    fn create_default_config(&mut self);
    fn save_parameters(&self) -> Result<(), String>;
    fn load_parameters(&mut self) -> Result<(), String>;
    fn get_existing_config_names() -> Result<Vec<String>, String>;
}
