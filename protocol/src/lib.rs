pub mod error;
pub mod host;
mod mu_frame;

use crate::{error::FrameDecodeError, mu_frame::MUFrame};
use error::{ProtoRecvError, ProtoSendError};
use std::io::{Read, Write};

/// Отправляет четыре байта `data.len()`, а потом сами данные.
fn send_proto_message<Data: AsRef<str>, Writer: Write>(
    data: MUFrame,
    mut writer: Writer,
) -> Result<(), ProtoSendError> {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

/// Читает четыре байта длины, а потом сами данные.
fn recv_proto_message<Reader: Read>(mut reader: Reader) -> Result<MUFrame, ProtoRecvError> {
    let mut raw_message = Vec::new();

    // Чтение prefix, len & opcode
    let mut buf = [0; 3];
    reader.read_exact(&mut buf)?;
    let payload_length = buf[1] as u8;
    raw_message.append(&mut buf.to_vec());

    // Чтение payload
    let mut buf = vec![0; payload_length as _];
    reader.read_exact(&mut buf)?;
    raw_message.append(&mut buf.to_vec());

    // Чтение crc & postfix
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    raw_message.append(&mut buf.to_vec());

    MUFrame::deserialize(&raw_message)
        .map_err(|_| ProtoRecvError::Decode(FrameDecodeError::BadEncoding))
}
