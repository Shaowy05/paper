use std::{fmt, time::Duration, path::Iter, io::stdout};

use crossterm::{
    terminal,
    event,
    event::{KeyEvent, Event, KeyCode}, execute,
};

use clap::{Parser, Subcommand};

// CONSTANTS
// Stores the number of milliseconds before returning a result.
const KEY_SAMPLE_RATE: u64 = 500;

// CLI Related {

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

// CLI Related }

// Editor Related {

pub struct Editor {
    paper: Paper
}

impl Editor {
    pub fn new(paper: Paper) -> Self {
        Self { paper } 
    }

    pub fn run(&self) {
        terminal::enable_raw_mode().expect("Failed to enable raw mode.");

        while self.process_keypress().unwrap_or_else(|arg| false) {};
    }

    fn process_keypress(&self) -> crossterm::Result<bool> {

        let key_event = self.read_key()?;

        match &key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: event::KeyEventKind::Press,
                ..
            } => return Ok(false),
            KeyEvent {
                kind: event::KeyEventKind::Press,
                ..
            } => {
                println!("{:?}", key_event);

                self.refresh_screen();
                return Ok(true)
            }
            _ => { return Ok(true); }
        }
    }

    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(
                Duration::from_millis(KEY_SAMPLE_RATE)
            )? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event)
                }
            }
        }
    }

    fn refresh_screen(&self) {
        execute!(stdout(), terminal::Clear(terminal::ClearType::Purge))
            .expect("Failed to clear the screen");

        let line_iter = self.paper.get_line_iter();
        for line in line_iter {
            println!("{line}\r");
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode()
            .expect("Failed to disable raw mode.");
    }
}

// Editor Related }

// Paper Related {

pub struct Paper {
    lines: Vec<Line>
}

impl Paper {
    pub fn new() -> Self {
        Paper { lines: Vec::new() }
    }

    pub fn add_empty_line(&mut self) {
        self.lines.push(Line::new());
    }

    pub fn add_line_from(&mut self, characters: Vec<char>) {
        self.lines.push(Line::from(characters));
    }

    pub fn get_line(&self, index: usize) -> &Line {
        self.lines.get(index).expect("No such line at index")
    }

        pub fn get_line_iter(&self) -> core::slice::Iter<'_, Line> {
            self.lines.iter()
    }

    pub fn get_line_mut(&mut self, index: usize) -> &Line {
        self.lines.get_mut(index).expect("No such line at index")
    }

}

pub struct Line {
    text: Vec<char>
}

impl Line {
    pub fn new() -> Self {
        Line { text: Vec::new() }
    }

    pub fn from(text: Vec<char>) -> Self {
        Line { text }
    }

    pub fn add_char_u8(&mut self, character: &char) {
        self.text.push(*character);
    }

}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let as_string: String = self.text.iter().collect();

        write!(f, "{as_string}")
    }
}

// Paper Related }