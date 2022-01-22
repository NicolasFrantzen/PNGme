use pngme::commands;
use pngme::args::PngMeArgs;

use anyhow::Result;
use clap::{AppSettings, Parser};


/// Encode a super secret message in a PNG!
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: PngMeArgs,
}


fn main() -> Result<()>
{
    let cli = Cli::parse();

    match &cli.command {
        PngMeArgs::Encode(args) => {
            commands::encode(args)?;
        }
        PngMeArgs::Decode(args) => {
            commands::decode(args)?;
        }
        PngMeArgs::Remove(args) => {
            commands::remove(args)?;
        }
        PngMeArgs::Print(args) => {
            commands::print_chunks(args)?;
        }
    }

    Ok(())
}
