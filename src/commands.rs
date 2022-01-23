use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

use anyhow::{Result, bail};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use std::fs;
use std::str::FromStr;

mod encdec
{
    use super::*;

    pub fn encrypt_message(key: &str, message: &str) -> String
    {
        let mc = new_magic_crypt!(key, 256);
        let base64 = mc.encrypt_str_to_base64(message);

        base64
    }


    pub fn decrypt_message(key: &str, base64: &str) -> Result<String>
    {
        let mc = new_magic_crypt!(key, 256);
        match mc.decrypt_base64_to_string(&base64)
        {
            Ok(message) => Ok(message),
            Err(_) => bail!("Failed to decrypt message. Maybe the key was wrong?")
        }
    }
}


/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArgs) -> Result<()>
{
    let content = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&content[..])?;

    let chunk_type = ChunkType::from_str(&args.chunk_type)?;

    let data: Vec<u8> = match &args.encryption_key {
        Some(key) => encdec::encrypt_message(&key, &args.message).as_bytes().to_vec(),
        None => args.message.clone().as_bytes().to_vec(),
    };

    if chunk_type.is_valid()
    {
        let chunk = Chunk::new(chunk_type, data);
        png.append_chunk(chunk);
    }
    else
    {
        bail!("Invalid chunk type!");
    }

    match &args.out_file {
        Some(out_file) => fs::write(out_file.clone(), png.as_bytes())?,
        None => println!("{png}"),
    };

    Ok(())
}


/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArgs) -> Result<()>
{
    let content = fs::read(args.file_path.clone())?;
    let png = Png::try_from(&content[..])?;

    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => {
            let data = chunk.data_as_string()?;
            let message = match &args.decryption_key {
                Some(key) => encdec::decrypt_message(key, &data)?,
                None => data,
            };

            println!("The secret message is: {message}")
        }
            ,
        None => bail!("Chunk type not found."),
    };

    Ok(())
}


/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> Result<()>
{
    let content = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&content[..])?;

    png.remove_chunk(&args.chunk_type)?;

    Ok(())
}


/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArgs) -> Result<()>
{
    let content = fs::read(args.file_path.clone())?;
    let png = Png::try_from(&content[..])?;
    println!("{png}");

    Ok(())
}
