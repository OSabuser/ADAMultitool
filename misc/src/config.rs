pub trait ConfigIO {
    /// Создание конфига с параметрами по умолчанию
    fn create_new(name: &str) -> Result<Self, String>
    where
        Self: Sized;

    /// Загрузка параметров существующего конфига с именем name
    fn create_from_existing(name: &str) -> Result<Self, String>
    where
        Self: Sized;

    fn get_config_name(&self) -> String;

    /// Сохранение параметров конфига в файл с именем self.name
    fn save_parameters(&self) -> Result<(), String>;
    /// Загрузка параметров конфига из файла с именем self.name
    fn load_parameters(&mut self) -> Result<(), String>;
    /// Список существующих конфигов
    fn list_existing_configs() -> Result<Vec<String>, String>;
}

enum _ConfigIOError {}
