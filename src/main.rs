use crossterm::{
    terminal,
    event
};

use crossterm::event:: {
    Event,
    KeyCode,
    KeyEvent
};

use clap::Parser;
use std::string::ParseError;
use std::{ io, io::Read };

use paper::{
    Cli,
    Commands::*,
    Paper,
    Line,
    Editor
};

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

    let mut paper: Paper = Paper::new();
    paper.add_line_from(vec![
        'h', 'e', 'l', 'l', 'o'
    ]);
    paper.add_line_from(vec![
        'w', 'o', 'r', 'l', 'd'
    ]);

    // Instantiating the Editor
    let editor = Editor::new(paper);

    // Running the Editor
    editor.run();

}