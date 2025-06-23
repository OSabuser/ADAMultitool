use log::warn;

use crate::mu_frame::MUFrame;
use std::time::Duration;

pub struct HostClient {
    serial_port: Box<dyn serialport::SerialPort + 'static>,
}

impl HostClient {
    /// Подключение к устройству
    pub fn connect(
        port_name: &str,
        baudrate: u32,
        timeout: Duration,
    ) -> Result<HostClient, String> {
        let serial_port = serialport::new(port_name, baudrate)
            .timeout(timeout)
            .open()
            .expect(format!("Unable to open: {}", port_name).as_str());

        Self::try_handshake(serial_port)
    }

    /// Попытка установить соединение с устройством
    fn try_handshake(instance: Box<dyn serialport::SerialPort + 'static>) -> Result<Self, String> {
        let mut attempts: u8 = 1;

        let mut client_connection = HostClient {
            serial_port: instance,
        };

        // Цикл попыток установить соединение
        'handshake_loop: loop {
            warn!("Attempting to handshake: {} times", attempts);

            let answer = client_connection
                .send_request("hello")
                .map_err(|e| e.to_string())?;

            warn!("Responce from device: {}", answer);

            if answer.as_bytes() == b"Hi!\r\n" {
                return Ok(client_connection);
            }
            attempts += 1;

            if attempts > 3 {
                break 'handshake_loop;
            }
        }
        return Err("Handshake failed!".to_string());
    }

    /// Отправка запроса на устройство
    pub fn send_request(&mut self, request: &str) -> Result<String, String> {
        let mut frame = MUFrame::new();
        frame
            .set_data(format!("{}{}", request, "\n").as_bytes().to_vec())
            .map_err(|e| e.to_string())?;
        crate::send_proto_message(frame, &mut self.serial_port)?;

        let new_frame =
            crate::recv_proto_message(&mut self.serial_port).map_err(|e| e.to_string())?;

        Ok(String::from_utf8(new_frame.get_data().to_vec()).map_err(|e| e.to_string())?)
    }
}
