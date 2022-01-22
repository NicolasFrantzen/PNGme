use std::path::PathBuf;
use clap::{Subcommand, Args, AppSettings};

#[derive(Subcommand)]
pub enum PngMeArgs
{
    /// Encode a message
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Encode(EncodeArgs),

    /// Decode a message
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decode(DecodeArgs),

    /// Remove a message
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Remove(RemoveArgs),

    /// Display a message
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Print(PrintArgs),
}


#[derive(Args)]
pub struct EncodeArgs
{
    /// Input file to encode from
    #[clap(parse(from_os_str))]
    pub file_path: PathBuf,

    /// Chunk type of the hidden message
    pub chunk_type: String,

    /// The hidden message
    pub message: String,

    /// Output file of the encoded PNG
    pub out_file: Option<PathBuf>,
}


#[derive(Args)]
pub struct DecodeArgs
{
    /// Input file to decode from
    #[clap(parse(from_os_str))]
    pub file_path: PathBuf,

    /// Chunk type of the hidden message to decode
    pub chunk_type: String,
}


#[derive(Args)]
pub struct RemoveArgs
{
    /// Input file to remove a secret message from
    #[clap(parse(from_os_str))]
    pub file_path: PathBuf,

    /// Chunk type of the hidden message to remove
    pub chunk_type: String,
}


#[derive(Args)]
pub struct PrintArgs
{
    /// Input file to print PNG chunks from
    #[clap(parse(from_os_str))]
    pub file_path: PathBuf,
}
