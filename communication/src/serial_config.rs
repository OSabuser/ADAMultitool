use configparser::ini::Ini;
use misc::config::ConfigIO;
use std::{fmt::Display, fs};

// TODO: TIMEOUT
// TODO: from, newtype pattern, tests

#[derive(Debug, Clone)]
pub struct PortConfig {
    config_name: String,
    port_name: String,
    baud_rate: u32,
}

impl PortConfig {
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
    fn create_new(name: &str) -> Result<Self, String> {
        let config = Self {
            config_name: name.to_string(),
            port_name: "/dev/ttyAMA0".to_string(),
            baud_rate: 9600,
        };
        //config.save_parameters()?;
        Ok(config)
    }

    fn get_config_name(&self) -> String {
        self.config_name.clone()
    }

    fn create_from_existing(name: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut config = Self {
            config_name: name.to_string(),
            port_name: "/dev/ttyAMA0".to_string(),
            baud_rate: 9600,
        };
        config.load_parameters()?;
        Ok(config)
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

impl Display for PortConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n Config_name: {}.ini \n Port name: {}, \n Baud rate: {}",
            self.config_name, self.port_name, self.baud_rate
        )
    }
}
