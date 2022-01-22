use args::PngMeArgs;
mod args;

use anyhow::Result;
use clap::{AppSettings, Parser};


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
    let _cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    /*match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {:?}", name)
        }
    }*/

    Ok(())
}
