use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use log::{Level, Metadata, Record};

pub struct CliLogger<W: Write + Send + Sync> {
    level: Level,
    writer: Arc<Mutex<W>>,
}

impl<W: Write + Send + Sync> CliLogger<W> {
    pub fn new(level: Level, writer: W) -> Self {
        Self {
            level,
            writer: Arc::new(Mutex::new(writer)),
        }
    }
}

impl<W: Write + Send + Sync> log::Log for CliLogger<W> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let color = match record.level() {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::Blue,
            _ => Color::Green,
        };

        // Must use a variable to make the lock safe.
        let mut writer = self.writer.lock().unwrap();
        execute!(
            writer,
            SetForegroundColor(color),
            Print(format!("{} - {}", record.level(), record.args())),
            ResetColor,
            Print("\n"),
        )
        .unwrap();
    }

    fn flush(&self) {}
}

use tui::{
    style::Style,
    text::{Span, Spans},
};
pub struct TuiLogger<'a> {
    pub message: Arc<Mutex<Vec<Spans<'a>>>>,
    level: Level,
}

impl TuiLogger<'_> {
    pub fn new(level: Level) -> Self {
        Self {
            message: Arc::new(Mutex::new(Vec::new())),
            level,
        }
    }

    pub fn get_message(&self) -> Vec<Spans> {
        self.message.lock().unwrap().clone()
    }
}

impl log::Log for TuiLogger<'_> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        use tui::style::Color;
        let message = format!("{} - {}", record.level(), record.args());
        let color = match record.level() {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::Blue,
            _ => Color::Green,
        };
        let span = Span::styled(message, Style::default().fg(color));

        let mut log = self.message.lock().unwrap();
        log.push(Spans::from(span));
    }

    fn flush(&self) {}
}
