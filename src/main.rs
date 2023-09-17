// use clap::{Parser, Subcommand};

// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct FessoraArgs {
//     /// Name of the file to process (only .pdfs at the moment)
//     #[clap(subcommand)]
//     mode: FessoraMode,
// }

// #[derive(Debug, Subcommand)]
// enum FessoraMode {
//     // Process a file and prepare to discuss it.
//     Prepare(PrepareCommand)
// }

// struct PrepareCommand {}

// struct Args {
//     /// Name of the file to process (only .pdfs at the moment)
//     #[arg(short, long)]
//     file: String,

//     /// Name of the output
//     #[arg(short, long)]
//     output: String,
// }

// fn main() {
//     let args = Args::parse();
//     dbg!(args);
// }

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add(AddArgs),
}

#[derive(Args)]
struct AddArgs {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add(name) => {
            println!("'myapp add' was used, name is: {:?}", name.name)
        }
    }
}