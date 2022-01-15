#![allow(dead_code, unused_variables)]

use crate::chunk_type::ChunkType;

use anyhow::{Result, Error, bail};

use std::fmt;


struct Chunk
{
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}


impl Chunk
{
    fn new(data: &[u8]) -> Result<Self>
    {
        let length_array: [u8; 4] = data[0..4].try_into()?;
        let chunk_type_array: [u8; 4] = data[4..8].try_into()?;
        let data_array: &[u8] = &data[8..data.len() - 4];
        let crc_array: [u8; 4] = data[data.len() - 4..data.len()].try_into()?;

        let length = u32::from_be_bytes(length_array);
        let chunk_type: ChunkType = ChunkType::try_from(chunk_type_array)?;
        let data_vec: Vec<u8> = data_array.to_vec();
        let crc = u32::from_be_bytes(crc_array);

        let new_chunk = Self {length, chunk_type, data: data_vec, crc};

        if crc != new_chunk.crc()
        {
            bail!("Invalid CRC")
        }

        Ok(new_chunk)
    }

    fn length(&self) -> u32
    {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType
    {
        &self.chunk_type
    }

    fn data(&self) -> &[u8]
    {
        &self.data
    }

    fn crc(&self) -> u32
    {
        crc::crc32::checksum_ieee(&self.as_bytes())
    }

    fn data_as_string(&self) -> Result<String>
    {
        Ok(String::from_utf8(self.data.clone())?)
    }

    fn as_bytes(&self) -> Vec<u8>
    {
        let result: Vec<u8> = self.chunk_type
            .bytes()
            .iter()
            .cloned()
            .chain(self.data.iter().cloned())
            .collect();

        result
    }
}


impl TryFrom<&[u8]> for Chunk
{
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self>
    {
        Ok(Self::new(&bytes)?)
    }
}


impl fmt::Display for Chunk
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}


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