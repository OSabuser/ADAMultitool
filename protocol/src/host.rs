use log::info;

use crate::mu_frame::MUFrame;
use std::time::Duration;

pub struct HostClient {
    serial_port: Box<dyn serialport::SerialPort + 'static>,
}

impl HostClient {
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

    fn try_handshake(
        mut instance: Box<dyn serialport::SerialPort + 'static>,
    ) -> Result<Self, String> {
        let mut frame = MUFrame::new();

        // TODO: 3 attempts

        frame
            .set_data(b"hello\n".to_vec())
            .map_err(|e| e.to_string())?;

        crate::send_proto_message(frame, &mut instance).map_err(|e| e.to_string())?;

        let answer = crate::recv_proto_message(&mut instance).map_err(|e| e.to_string())?;

        let string_data =
            String::from_utf8(answer.get_data().to_vec()).map_err(|e| e.to_string())?;

        info!("Received response: {}", string_data);

        if answer.get_data() != b"Hi!\r\n" {
            return Err("Handshake failed!".to_string());
        }

        Ok(Self {
            serial_port: instance,
        })
    }

    pub fn send_request(&mut self, request: &str) -> Result<String, String> {
        let mut frame = MUFrame::new();
        frame
            .set_data(format!("{}{}", request, "\n").as_bytes().to_vec())
            .map_err(|e| e.to_string())?;
        crate::send_proto_message(frame, &mut self.serial_port)?;

        let frame = crate::recv_proto_message(&mut self.serial_port).map_err(|e| e.to_string())?;
        Ok(String::from_utf8(frame.get_data().to_vec()).map_err(|e| e.to_string())?)
    }
}
