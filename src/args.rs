#![allow(dead_code, unused_imports)]

use std::path::PathBuf;
use clap::{Subcommand, Parser, Args};

#[derive(Subcommand)]
pub enum PngMeArgs
{
    /// Encode a message
    Encode(EncodeArgs),

    /// Decode a message
    Decode(DecodeArgs),

    /// Remove a message
    Remove(RemoveArgs),

    /// Display a message
    Print(PrintArgs),
}


#[derive(Args)]
pub struct EncodeArgs
{
    #[clap(parse(from_os_str))]
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    out_file: Option<String>,
}


#[derive(Args)]
pub struct DecodeArgs
{
    #[clap(parse(from_os_str))]
    file_path: PathBuf,
    chunk_type: String,
}


#[derive(Args)]
pub struct RemoveArgs
{
    #[clap(parse(from_os_str))]
    file_path: PathBuf,
    chunk_type: String,
}


#[derive(Args)]
pub struct PrintArgs
{
    #[clap(parse(from_os_str))]
    file_path: PathBuf,
}
