mod cli;

use cli::{Commands, Cli};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Process(args) => {
            println!("'myapp add' was used, name is: {:?}", args.filename)
        }
    }
}