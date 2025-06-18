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
    ) -> Result<HostClient, serialport::Error> {
        let serial_port = serialport::new(port_name, baudrate)
            .timeout(timeout)
            .open()?;

        Ok(HostClient { serial_port })
    }

    fn try_handshake(
        mut instance: Box<dyn serialport::SerialPort + 'static>,
    ) -> Result<Self, serialport::Error> {
        let mut frame = MUFrame::new();
        frame.set_data(b"hello\n".to_vec()).unwrap();
        crate::send_proto_message(frame, &mut instance).unwrap();

        let buf = Vec::new();

        let answer = crate::recv_proto_message(&buf[..]).unwrap();

        Ok(Self {
            serial_port: instance,
        })
    }
}
