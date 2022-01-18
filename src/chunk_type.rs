#![allow(dead_code)]

use anyhow::{Result, Error, bail};

use std::str::FromStr;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType
{
    data: [u8; 4],
}


impl ChunkType
{
    fn new(data: [u8; 4]) -> Self
    {
        Self {data}
    }

    pub fn bytes(&self) -> [u8; 4]
    {
        self.data.clone()
    }

    pub fn is_valid_byte(byte: u8) -> bool
    {
        byte.is_ascii_lowercase() || byte.is_ascii_uppercase()
    }

    pub fn is_valid(&self) -> bool
    {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool
    {
        self.data[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool
    {
        self.data[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool
    {
        self.data[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool
    {
        self.data[3].is_ascii_lowercase()
    }
}


impl TryFrom<[u8; 4]> for ChunkType
{
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self>
    {
        Ok(Self::new(bytes))
    }
}


impl FromStr for ChunkType
{
    type Err = Error;

    fn from_str(str: &str) -> Result<Self>
    {
        let bytes: [u8; 4] = str.as_bytes().try_into()?;

        for i in bytes
        {
            if !Self::is_valid_byte(i)
            {
                bail!("Invalid byte: {}", i);
            }
        }

        Ok(Self::new(bytes))
    }
}


impl fmt::Display for ChunkType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let chunk_vec = self.data.to_vec();
        let chunk_str = String::from_utf8(chunk_vec).expect("Found invalid UTF-8");

        write!(f, "{chunk_str}")
    }
}


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
