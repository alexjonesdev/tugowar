use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::{error::Error, io};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode, self};
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

fn main() {
    println!("Hello, world!");
}