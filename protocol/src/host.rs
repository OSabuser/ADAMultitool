use crate::error;
use crate::{error::DeviceConnectError, error::ProtoRecvError, error::ProtoSendError};
use crate::{error::FrameDecodeError, mu_frame::MUFrame};
use std::time::Duration;

pub struct HostClient {
    serial_port: Box<dyn serialport::SerialPort + 'static>,
}

impl HostClient {
    pub fn connect(
        port_name: &str,
        baudrate: u32,
        timeout: Duration,
    ) -> Result<HostClient, error::DeviceConnectError> {
        let serial_port = serialport::new(port_name, baudrate)
            .timeout(timeout)
            .open()
            .expect(format!("Unable to open: {}", port_name).as_str());

        Self::try_handshake(serial_port)
    }

    fn try_handshake(
        mut instance: Box<dyn serialport::SerialPort + 'static>,
    ) -> Result<Self, error::DeviceConnectError> {
        let mut frame = MUFrame::new();

        frame
            .set_data(b"hello\n".to_vec())
            .map_err(|_| DeviceConnectError::BadHandshake.into())?;

        crate::send_proto_message(frame, &mut instance)
            .map_err(|_| DeviceConnectError::BadHandshake.into())?;

        let buf = Vec::new();

        let answer = crate::recv_proto_message(&buf[..])
            .map_err(|_| DeviceConnectError::BadHandshake.into())?;

        if answer.get_data() != b"Hi!\r\n" {
            return Err(DeviceConnectError::BadHandshake.into());
        }

        Ok(Self {
            serial_port: instance,
        })
    }
}
