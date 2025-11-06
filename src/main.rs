//use crossterm::terminal::disable_raw_mode;
//use crossterm::terminal::enable_raw_mode;
//use std::io::{self, Read};
//no longer used moved to editor.rs
mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}
