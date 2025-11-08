use crate::Document;
use crate::Row;
use crate::Terminal;
use std::time::Duration;
use std::time::Instant;
use termion::color;
use termion::event::Key;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(145, 144, 144);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(222, 222, 222);
const VERSION: &str = env!("CARGO_PKG_VERSION");
//u8 is a specific type of int
const QUIT_TIMES: u8 = 1;

#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
//will use either to see if it is saved or to indicate whether file has changed that are not on disk

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    //lol
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
    highlighted_word: Option<String>,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    //default function called when nothing else is specified (this is in java aswell)
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut inital_status =
            String::from("control-f to find / control-s to save / control-q to quit");

        //why does it force me to have it like this when i save the file?
        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);

            if let Ok(doc) = doc {
                doc
            }

            else {
                initial_status = format!("wd40 cannot open it. The file: {}", file_name);
                Document::default();
            }

            else {
                Document::default()
            };

            Self {
                should_quit: false,
                terminal: Terminal::default().expect("Terminal failed. Use Bash or Powershell 7.x+.");
                document,
                cursor_position: Position::default(),
                status_message: StatusMessage::from(initial_status),
                quit_times: QUIT_TIMES,
                highlighted_word: None,
            }
        }; //why does it want a semicolon?
    }
}
