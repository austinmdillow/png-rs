
use crate::chunk_type::ChunkType;

use std::fmt;
use crc;

#[derive(Debug, PartialEq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {

        let crc_data = [&chunk_type.bytes(), data.as_slice()].concat();
        let crc = crc::crc32::checksum_ieee(&crc_data);
        return Chunk { length: data.len() as u32, chunk_type: chunk_type, data: data, crc: crc };
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
        return Ok("tostring".to_string());
        //Ok(str_rep.to_string());
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }


}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str; // how does this work?

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        let length_array: [u8;4] = value[0..4].try_into().expect("Can't read length byte array");
        let length = u32::from_be_bytes(length_array);

        let type_array: [u8;4] = value[4..8].try_into().expect("Can't read length byte array");
        let chunk_type = ChunkType::try_from(type_array).unwrap();

        println!("Chunk type is {chunk_type} . validity {}", chunk_type.is_valid());

        let mut data_vec = Vec::new();

        for i in 8..value.len() {
            data_vec.push(value[i]);
        }
        if data_vec.len() != length as usize {
            return Err("Expected encoded length to equal input length");
        }
        
        let crc_start_idx = length as usize + 8usize;
        let encoded_crc_array:[u8;4] = value[crc_start_idx..crc_start_idx+4].try_into().expect("Can't read crc byte array");
        let encoded_crc = u32::from_be_bytes(encoded_crc_array);

        
        let mut new_chunk = Chunk::new(chunk_type, data_vec);

        if new_chunk.crc != encoded_crc {
            return Err("Crcs do not match");
        }
        println!("Chunk len {length}.length");
        
        Ok(new_chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} data_len:{}",self.chunk_type, self.data.len())

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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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

