use rand::{thread_rng, Rng};
use std::io;
use std::io::Read;
use std::time::Instant;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

use crossterm::{terminal, event}
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub fn tui() {
    crossterm::terminal::enable_raw_mode()?;

}
