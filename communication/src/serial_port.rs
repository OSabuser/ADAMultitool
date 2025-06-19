/// Модуль для работы с последовательными портами
use std::time::Duration;

const SUPPORTED_BAUDATES: [u32; 5] = [9600, 19200, 38400, 57600, 115200];

pub struct SerialInterface {
    pub port_instance: Box<dyn serialport::SerialPort + 'static>,
    port_name: String,
}

impl SerialInterface {
    pub fn new(port_name: &str, baudrate: u32, timeout: Duration) -> SerialInterface {
        let port = serialport::new(port_name, baudrate)
            .timeout(timeout)
            .open()
            .expect(format!("Failed to open port: {}", port_name).as_str());

        SerialInterface {
            port_instance: port,
            port_name: port_name.to_string(),
        }
    }

    /// Получение списка доступных портов
    pub fn get_available_port_names() -> Result<Vec<String>, String> {
        if let Ok(ports) = serialport::available_ports() {
            return Ok(ports.iter().map(|p| p.port_name.clone()).collect());
        }

        Err("Unable to get port names".to_string())
    }

    /// Получение списка поддерживаемых скоростей
    pub fn get_supported_port_speed() -> Result<Vec<String>, String> {
        Ok(SUPPORTED_BAUDATES
            .into_iter()
            .map(|x| x.to_string())
            .collect())
    }

    /// Грязная запись без всяких проверок
    pub fn write_data_unsafe(&mut self, data: &[u8]) -> () {
        match self.port_instance.write(data) {
            _ => (),
        }
    }

    /// Отправка данных на интерфейсную плату
    pub fn write_data(&mut self, data: &[u8]) -> Result<usize, String> {
        if let Ok(size) = self.port_instance.write(data) {
            return Ok(size);
        }
        return Err(format!("Failed to write to port: {}", self.port_name));
    }

    /// Очистка входного буфера приемника
    pub fn clear_input_buffer(&mut self) -> Result<(), String> {
        if let Ok(_) = self.port_instance.clear(serialport::ClearBuffer::Input) {
            return Ok(());
        }
        return Err(format!("Failed to clear input buffer: {}", self.port_name));
    }

    pub fn get_available_bytes(&mut self) -> Result<u32, String> {
        if let Ok(bytes) = self.port_instance.bytes_to_read() {
            return Ok(bytes);
        }
        return Err(format!("Failed to get available bytes: {}", self.port_name));
    }

    /// Чтение данных от интерфейсной платы
    pub fn read_data(&mut self, data: &mut [u8]) -> Result<(), String> {
        if let Ok(_) = self.port_instance.read(data) {
            return Ok(());
        }
        return Err("Timeout has been reached".to_string());
    }
}
