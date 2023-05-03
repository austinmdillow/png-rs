use crate::chunk_type::ChunkType;

use crc;
use std::{
    fmt,
    fs::read,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, PartialEq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc_data = [&chunk_type.bytes(), data.as_slice()].concat();
        let crc = Self::crc_checksum(&crc_data);
        return Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            data: data,
            crc: crc,
        };
    }

    fn crc_checksum(bytes: &[u8]) -> u32 {
        crc::crc32::checksum_ieee(&bytes)
    }

    pub fn length(&self) -> u32 {
        return self.data.len() as u32;
    }

    pub fn chunk_type(&self) -> &ChunkType {
        return &self.chunk_type;
    }

    pub fn data(&self) -> &[u8] {
        return &self.data;
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, std::str::Utf8Error> {
        let str_rep = std::str::from_utf8(&self.data)?;
        return Ok(str_rep.to_string());
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let length_bytes = self.length.to_be_bytes();

        let type_bytes = self.chunk_type.bytes();

        output.extend_from_slice(&length_bytes);
        output.extend_from_slice(&type_bytes);
        let mut data_copy = self.data.clone();
        output.append(&mut data_copy);

        let crc_bytes = self.crc.to_be_bytes();
        output.extend_from_slice(&crc_bytes);

        return output;
    }

    pub fn read_chunk(reader: &mut BufReader<&[u8]>) -> Result<Chunk, String> {
        let mut buffer = [0; 4];

        reader.read_exact(&mut buffer).map_err(|_x| "Error".to_string())?;

        let length = u32::from_be_bytes(buffer);

        reader
            .read_exact(&mut buffer)
            .map_err(|_x| "Error".to_string())?;
        let chunk_type:ChunkType = buffer.try_into()?;

        let mut data = vec![0; length as usize];
        reader
            .read_exact(&mut data)
            .map_err(|_x| "Error".to_string())?;

        reader
            .read_exact(&mut buffer)
            .map_err(|_x| "Error".to_string())?;
        let decoded_crc = u32::from_be_bytes(buffer);

        let crc_data = [&chunk_type.bytes(), data.as_slice()].concat();
        let crc = Self::crc_checksum(&crc_data);

        if decoded_crc != crc {
            return Err("Decoded crc does not match data crc".to_string());
        }

        Ok(Chunk {
            length: length,
            chunk_type: chunk_type,
            data: data,
            crc: crc,
        })
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = String; // how does this work?

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let chunk = Self::read_chunk(&mut reader)?;

        if !reader.fill_buf().unwrap().is_empty() {
            return Err("invalid chunk".to_string());
        }

        println!("Chunk len {}",chunk.length());

        Ok(chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chunk {} data_len:{} crc:{}", self.chunk_type, self.data.len(), self.crc())
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        println!("{}", chunk);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
