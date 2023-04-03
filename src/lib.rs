use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn get_command(&self) -> &Option<Commands> {
        &self.command
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds a new sheet of paper.
    Add,

    /// 'Bins' the current sheet of paper or the current sheet if not
    /// specified in the command. 
    Bin {
        number: Option<usize>,

        /// Delete all sheets of paper
        #[arg(short, long)]
        all: bool,
    },

    /// Switches to the sheet of paper dictated by the number given.
    Switch { number: usize },

    /// Lists all the active sheets of paper
    List,
}

