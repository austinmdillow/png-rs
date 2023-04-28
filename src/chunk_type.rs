use std::fmt;
use std::str;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ChunkType {
    bytes: [u8; 4],
}

// check bit 4 of byte 1 ... critical
impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() {
            return false;
        }

        return true;
    }

    fn is_critical(&self) -> bool {
        (self.bytes[0] & 1 << 5) == 0
    }

    fn is_public(&self) -> bool {
        (self.bytes[1] & 1 << 5) == 0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        (self.bytes[2] & 1 << 5) == 0
    }

    fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] & 1 << 5) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str; // how does this work?

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("Expected size of 4");
        }

        let mut bytes: [u8; 4] = [0, 0, 0, 0];

        for i in 0..4 {
            let c = s.bytes().nth(i);

            match c {
                Some(val) => {
                    if !val.is_ascii_alphabetic() {
                        return Err("Non Alphabetic character");
                    }
                    bytes[i] = val;
                }

                None => return Err("Could not decode byte"),
            }
        }

        let proposed_chunk = ChunkType { bytes: bytes };

        return Ok(proposed_chunk);
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = str::from_utf8(&self.bytes).unwrap();
        write!(f, "{}", output)
    }
}

//// Unit Tests
///

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        println!("AS STR{}", &chunk.to_string());
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
