pub mod error;
pub mod host;
mod mu_frame;

use crate::{error::FrameDecodeError, mu_frame::MUFrame};
use error::{ProtoRecvError, ProtoSendError};
use std::io::{Read, Write};

/// Отправка сообщения
fn send_proto_message<Writer: Write>(
    data: MUFrame,
    mut writer: Writer,
) -> Result<(), ProtoSendError> {
    let bytes = data.serialize();
    writer.write_all(&bytes)?;

    Ok(())
}

/// Прием сообщения
fn recv_proto_message<Reader: Read>(mut reader: Reader) -> Result<MUFrame, ProtoRecvError> {
    let mut raw_message = Vec::new();

    // Чтение prefix, len & opcode
    let mut buf = [0; 3];
    reader.read_exact(&mut buf)?;
    let payload_length = buf[1] + 1 as u8;
    raw_message.append(&mut buf.to_vec());

    // Чтение payload
    let mut buf = vec![0; payload_length as _];
    reader.read_exact(&mut buf)?;
    raw_message.append(&mut buf.to_vec());

    // Чтение crc & postfix
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    raw_message.append(&mut buf.to_vec());

    Ok(MUFrame::deserialize(&raw_message).map_err(FrameDecodeError::from)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_and_recv() {
        let mut frame_to_send = MUFrame::new();
        frame_to_send
            .set_data(b"get server_info\n".to_vec())
            .unwrap();

        let mut buf = Vec::new();

        send_proto_message(frame_to_send.clone(), &mut buf).unwrap();

        let received_frame = recv_proto_message(&buf[..]).unwrap();
        assert_eq!(received_frame.get_data(), frame_to_send.get_data());
        assert_eq!(received_frame, frame_to_send);
    }
}
