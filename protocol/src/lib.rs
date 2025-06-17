pub mod error;
pub mod host;

use error::{ProtoRecvError, ProtoSendError};
use std::io::{Read, Write};

/// Отправляет четыре байта `data.len()`, а потом сами данные.
fn send_proto_string<Data: AsRef<str>, Writer: Write>(
    data: Data,
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
fn recv_proto_string<Reader: Read>(mut reader: Reader) -> Result<String, ProtoRecvError> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| ProtoRecvError::BadEncoding)
}
