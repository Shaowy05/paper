use clap::Parser;

use paper::{Cli, Commands::*};

fn main() {

    // Parsing CLI arguments
    let cli = Cli::parse();

    // If there was no subcommand, then we run the default behaviour and
    // open the last sheet of paper that was used.
    let cli_command = cli.get_command();
    if let Some(command) = cli_command {
        match command {
            Bin {number, all} => {
                println!("{all}");
            }
            _ => println!("Some other command")
        }
    }
    else {
        println!("No command passed in!");
    }
}