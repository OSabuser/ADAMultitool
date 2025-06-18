pub mod error;
pub mod host;
pub mod mu_frame;

use crate::{error::FrameDecodeError, mu_frame::MUFrame};
use error::{ProtoRecvError, ProtoSendError};
use std::{
    io::{Read, Write},
    thread,
};

/// Задержка приема ответа
const ANSWER_DELAY_MS: u64 = 150;

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
    let mut read_buffer = [0; 256];

    thread::sleep(std::time::Duration::from_millis(ANSWER_DELAY_MS));

    // Чтение отклика от интерфейсной платы
    reader.read(&mut read_buffer)?;

    todo!("Парсинг принятых байт");

    Ok(MUFrame::deserialize(&read_buffer).map_err(FrameDecodeError::from)?)
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
