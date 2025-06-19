use core::error;
use log::error;
use std::fmt::Display;

const SYNC1: u8 = 0xAA;
const SYNC2: u8 = 0xBB;
const MAX_DATA_SIZE: u8 = u8::MAX;
const CONSOLE_OPCODE: u8 = 0xC0;

/// Пакет данных протокола "МЮ" и методы работы с ним
///
///
/// ## Пример
/// ```ignore
/// let mut frame_to_send = MUFrame::new();
///       frame_to_send
///       .set_data(b"get server_info\n".to_vec())?;
/// let raw_bytes = frame_to_send.serialize();
/// ```
///

#[derive(Debug, PartialEq, Clone)]
pub struct MUFrame {
    prefix: u8,
    length: u8,
    opcode: u8,
    data: Vec<u8>,
    crc: u8,
    suffix: u8,
}

impl MUFrame {
    pub fn new() -> Self {
        Self {
            prefix: SYNC1,
            length: 0,
            opcode: CONSOLE_OPCODE,
            data: Vec::with_capacity(MAX_DATA_SIZE as usize),
            crc: 0,
            suffix: SYNC2,
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Загрузка данных в фрейм, вычисление CRC и длины
    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), String> {
        if data.len() > MAX_DATA_SIZE as usize || data.is_empty() {
            return Err("Data too long".to_string());
        }

        if !data.is_ascii() {
            return Err("Bad encoding".to_string());
        }

        self.length = data.len() as u8;
        self.data = data;
        self.crc = self.calculate_src();

        Ok(())
    }

    /// Десериализация данных из буфера
    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        let mut frame = Self::new();
        frame.prefix = data[0];
        frame.length = data[1];
        frame.opcode = data[2];

        frame.data = data[3..3 + frame.length as usize].to_vec();
        frame.crc = data[3 + frame.length as usize];
        frame.suffix = data[4 + frame.length as usize];

        frame.invalidate_frame()?;

        Ok(frame)
    }

    /// Сериализация данных
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(5 + self.length as usize);
        result.push(self.prefix);
        result.push(self.length);
        result.push(self.opcode);
        result.extend(self.data.iter());
        result.push(self.crc);
        result.push(self.suffix);
        result
    }

    /// Проверка валидности фрейма
    fn invalidate_frame(&self) -> Result<(), String> {
        if !self.is_prefix_correct() {
            return Err("Bad prefix".to_string());
        }
        if !self.is_postfix_correct() {
            return Err("Bad postfix".to_string());
        }
        if !self.is_crc_valid(self.crc) {
            return Err("Bad CRC".to_string());
        }

        if !self.data.is_ascii() {
            return Err("Bad encoding".to_string());
        }

        Ok(())
    }

    /// Вычисление CRC
    fn calculate_src(&self) -> u8 {
        const CRC8_MU: crc::Crc<u8> = crc::Crc::<u8>::new(&crc::CRC_8_NRSC_5);

        let mut crc_data = Vec::with_capacity(self.length as usize + 1);
        crc_data.push(self.opcode);
        crc_data.extend(self.data.iter());

        CRC8_MU.checksum(&crc_data)
    }

    /// Проверка соответствия фактического CRC посылки с принятым
    fn is_crc_valid(&self, crc: u8) -> bool {
        let calculated_crc = self.calculate_src();
        calculated_crc == crc
    }

    /// Проверка корректности префикса (SYNC1)
    fn is_prefix_correct(&self) -> bool {
        self.prefix == SYNC1
    }

    /// Проверка корректности постфикса (SYNC2)
    fn is_postfix_correct(&self) -> bool {
        self.suffix == SYNC2
    }
}

impl Display for MUFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MU Message: \n 1.Opcode={}, \n 2.Data length={} \n 3.Payload={:?} \n 4.CRC={}",
            self.opcode, self.length, self.data, self.crc
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_crc_calculation() {
        let mut frame = MUFrame::new();
        frame.set_data(b"Hello, server!\n".to_vec()).unwrap();
        assert_eq!(frame.is_crc_valid(0xBD), true);
    }

    #[test]
    fn test_serialize() {
        let mut frame = MUFrame::new();

        frame.set_data(b"Test string!\n".to_vec()).unwrap();

        let serialized_vec = frame.serialize();
        assert_eq!(serialized_vec.len(), 5 + frame.length as usize);

        assert_eq!(
            serialized_vec,
            vec![
                0xAA, 0x0D, 0xC0, 0x54, 0x65, 0x73, 0x74, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67,
                0x21, 0x0A, 0x1E, 0xBB
            ]
        );
    }

    #[test]
    fn test_deserialize() {
        let serialized_vec = vec![
            0xAA, 0x0D, 0xC0, 0x54, 0x65, 0x73, 0x74, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67,
            0x21, 0x0A, 0x1E, 0xBB,
        ];
        let frame = MUFrame::deserialize(&serialized_vec).unwrap();
        assert_eq!(frame.is_prefix_correct(), true);
        assert_eq!(frame.is_postfix_correct(), true);
        assert_eq!(frame.is_crc_valid(0x1E), true);
        assert_eq!(frame.opcode, CONSOLE_OPCODE);
        assert_eq!(frame.data, b"Test string!\n");
    }

    #[test]
    fn test_frame_invalidation() {
        let serialized_vec = vec![
            0xAA, 0x0D, 0xC0, 0x54, 0x65, 0x73, 0x74, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E, 0x67,
            0x21, 0x0A, 0x1E, 0xBB,
        ];
        let frame = MUFrame::deserialize(&serialized_vec).unwrap();
        assert_eq!(frame.is_prefix_correct(), true);
        assert_eq!(frame.is_postfix_correct(), true);
        assert_eq!(frame.is_crc_valid(0x1E), true);
        assert_eq!(frame.opcode, CONSOLE_OPCODE);
        assert_eq!(frame.data, b"Test string!\n");
        frame.invalidate_frame().unwrap();
    }
}
