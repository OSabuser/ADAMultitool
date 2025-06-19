pub trait ConfigIO {
    /// Создание конфига с параметрами по умолчанию
    /// Сохранение конфига в файл ini
    fn create_default_config() -> Result<Self, String>
    where
        Self: Sized;

    /// Загрузка параметров существующего конфига с именем name
    fn create_from_existing_config(name: &str) -> Result<Self, String>
    where
        Self: Sized;

    /// Получение имени конфига, к которому относятся загруженные параметры
    fn get_actual_config_name(&self) -> String;
    /// Изменение имени конфига
    fn change_config_name(&mut self, name: &str) -> Result<(), String>;
    /// Сохранение параметров конфига в файл с именем self.name
    fn save_parameters(&self) -> Result<(), String>;
    /// Загрузка параметров конфига из файл с именем self.name
    fn load_parameters(&mut self) -> Result<(), String>;
    /// Список существующих конфигов
    fn list_existing_configs() -> Result<Vec<String>, String>;
}

enum _ConfigIOError {}
