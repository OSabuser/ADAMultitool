use configparser::ini::Ini;
use misc::config::ConfigIO;
use std::fs;

#[derive(Debug, Clone)]
pub struct PortConfig {
    config_name: String,
    port_name: String,
    baud_rate: u32,
}

impl PortConfig {
    pub fn new(config_path: &str) -> PortConfig {
        PortConfig {
            config_name: config_path.to_string(),
            port_name: "COM1".to_string(),
            baud_rate: 9600,
        }
    }

    pub fn get_port_name(&self) -> String {
        self.port_name.clone()
    }

    pub fn get_baud_rate(&self) -> u32 {
        self.baud_rate
    }

    pub fn set_port_name(&mut self, port_name: String) {
        self.port_name = port_name;
    }

    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.baud_rate = baud_rate;
    }
}

impl ConfigIO for PortConfig {
    fn create_default_config() -> Result<Self, String> {
        let config = PortConfig::new("default");
        config.save_parameters()?;
        Ok(config)
    }

    fn create_from_existing_config(name: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut config = Self {
            config_name: name.to_string(),
            port_name: "COM1".to_string(),
            baud_rate: 9600,
        };
        config.load_parameters()?;
        Ok(config)
    }

    fn get_actual_config_name(&self) -> String {
        self.config_name.clone()
    }

    fn change_config_name(&mut self, name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name should not be empty".into());
        }

        if name
            .chars()
            .all(|arg0: char| char::is_ascii_alphanumeric(&arg0))
        {
            self.config_name = name.to_string();
            return Ok(());
        }

        Err("Name should be alphanumeric".into())
    }

    fn load_parameters(&mut self) -> Result<(), String> {
        let mut config_instance = Ini::new();
        config_instance.load(format!("configs/serial/{}.ini", self.config_name))?;

        match config_instance.get("serial_settings", "PORT_NAME") {
            Some(name) => self.set_port_name(name),
            None => return Err("Unable to get port name".to_string()),
        };

        match config_instance.getuint("serial_settings", "BAUD_RATE") {
            Ok(Some(baud_rate)) => self.set_baud_rate(baud_rate as u32),
            _ => return Err("Unable to get baud rate".to_string()),
        };

        Ok(())
    }

    fn save_parameters(&self) -> Result<(), String> {
        let mut config_instance = Ini::new();

        config_instance.set("serial_settings", "PORT_NAME", Some(self.get_port_name()));
        config_instance.set(
            "serial_settings",
            "BAUD_RATE",
            Some(self.get_baud_rate().to_string()),
        );
        return config_instance
            .write(format!("configs/serial/{}.ini", self.config_name))
            .map_err(|e| e.to_string());
    }

    fn list_existing_configs() -> Result<Vec<String>, String> {
        let mut list_of_files = Vec::new();

        if let Ok(entries) = fs::read_dir("configs/serial/") {
            for entry in entries {
                if let Ok(dir) = entry {
                    list_of_files.push(
                        dir.path()
                            .display()
                            .to_string()
                            .replace("configs/serial/", "")
                            .replace(".ini", ""),
                    );
                } else {
                    break;
                }
            }
            return Ok(list_of_files);
        }

        return Err("Unable to get config file names".to_string());
    }
}
