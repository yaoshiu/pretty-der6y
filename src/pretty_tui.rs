use crate::pretty_logger::TuiLogger;

use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{cell::RefCell, error::Error, io::Write, rc::Rc, sync::Arc, time::Duration};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};

const TITLE: &str = r#"
____              _    _            ____               _            
|  _ \  _ __  ___ | |_ | |_  _   _  |  _ \   ___  _ __ | |__   _   _ 
| |_) || '__|/ _ \| __|| __|| | | | | | | | / _ \| '__|| '_ \ | | | |
|  __/ | |  |  __/| |_ | |_ | |_| | | |_| ||  __/| |   | |_) || |_| |
|_|    |_|   \___| \__| \__| \__, | |____/  \___||_|   |_.__/  \__, |
                             |___/                             |___/ 
"#;

const TITLE2: &str = r#"
____              _    _            ____               _            
|  _ \  _ __  ___ | |_ | |_  _   _  |  _ \   ___  _ __ | |__   _   _ 
| |_) || '__|/ _ \| __|| __|| | | | | | | | / _ \| '__|| '_ \ | | | |
|  __/ | |  |  __/| |_ | |_ | |_| | | |_| ||  __/| |   | |_) || |_| |
|_|    |_|   \___| \__| \__| \__, | |____/  \___||_|   |_.__/  \__, |
                             |___/                             |___/ 
"#;

const _TITLE3: &str = r#"
____              _    _           
|  _ \  _ __  ___ | |_ | |_  _   _  
| |_) || '__|/ _ \| __|| __|| | | | 
|  __/ | |  |  __/| |_ | |_ | |_| | 
|_|    |_|   \___| \__| \__| \__, | 
                             |___/  
  ____               _            
 |  _ \   ___  _ __ | |__   _   _ 
 | | | | / _ \| '__|| '_ \ | | | |
 | |_| ||  __/| |   | |_) || |_| |
 |____/  \___||_|   |_.__/  \__, |
                            |___/ 
"#;

enum InputMode {
    Editing,
    Normal,
}

enum Widget {
    Account,
    Password,
    Mileage,
    Time,
}

/// A TUI with a welcome page and main page
///
/// # Usage
///
/// Example
/// ```
/// let stdout = io::stdout();
/// let backend = CrosstermBackend::new(stdout);
/// let logger = TuiLogger::new(Level::Info);
/// let mut tui = Tui::new(backend, logger);
/// tui.welcome()?;
/// ```
pub struct Tui<'a, B: Backend> {
    account: String,
    cursorpos: u16,
    input_mode: InputMode,
    logger: Arc<TuiLogger<'a>>,
    mileage_percent: u16,
    password: String,
    time: String,
    selected: Widget,
    terminal: Rc<RefCell<Terminal<B>>>,
}

type PrettyTuiResult<A> = Result<A, Box<dyn Error>>;

impl<'a, B: Backend + Write> Tui<'a, B> {
    pub fn new(mut backend: B, logger: Arc<TuiLogger<'a>>) -> PrettyTuiResult<Self> {
        enable_raw_mode()?;
        execute!(backend, EnterAlternateScreen)?;

        let terminal = Terminal::new(backend)?;
        Ok(Self {
            cursorpos: 0,
            input_mode: InputMode::Normal,
            logger,
            terminal: Rc::new(RefCell::new(terminal)),
            account: String::new(),
            password: String::new(),
            mileage_percent: 100,
            selected: Widget::Account,
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }
}

impl<B: Backend + Write> Tui<'_, B> {
    pub fn quit(&self) -> PrettyTuiResult<()> {
        disable_raw_mode()?;
        let mut terminal = match self.terminal.try_borrow_mut() {
            Ok(t) => t,
            Err(_) => return Ok(()),
        };
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn welcome(&self) -> PrettyTuiResult<()> {
        loop {
            self.terminal
                .borrow_mut()
                .draw(|frame| self.ui_welcome(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    // For windows double read bug
                    continue;
                }
                break;
            }
        }

        Ok(())
    }

    fn ui_welcome(&self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .margin(2)
            .constraints([Constraint::Percentage(100)])
            .split(frame.size());
        let border = Block::default().borders(Borders::ALL);
        frame.render_widget(border.clone(), chunks[0]);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(8),
                    Constraint::Length(16),
                    Constraint::Max(u16::MAX),
                    Constraint::Length(4),
                    Constraint::Length(4),
                ]
                .as_ref(),
            )
            .split(border.inner(chunks[0]));

        let para = Paragraph::new(TITLE)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(para, chunks[1]);
        let para = Paragraph::new("Press any key to continue...")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green));
        frame.render_widget(para, chunks[3]);
    }

    pub fn main(&mut self) -> PrettyTuiResult<Option<(String, String, u16, String)>> {
        loop {
            {
                // WARN: Should make sure that the terminal dies immediately
                let mut terminal = self.terminal.borrow_mut();
                terminal.draw(|f| self.ui_main(f).unwrap())?;
            }
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        // For windows double read bug
                        continue;
                    }
                    match self.input_mode {
                        InputMode::Normal => {
                            if self.handle_normal(key.code).is_some() {
                                return Ok(None);
                            }
                        }
                        InputMode::Editing => {
                            if let Some(res) = self.handle_editing(key.code) {
                                return Ok(Some(res));
                            }
                        }
                    }
                }
            }
        }
    }

    // Some(()) for break the loop and None for continue
    fn handle_normal(&mut self, key: KeyCode) -> Option<()> {
        match key {
            KeyCode::Esc => Some(()),
            KeyCode::Up => {
                self.select(KeyCode::Up);
                None
            }
            KeyCode::Down => {
                self.select(KeyCode::Down);
                None
            }
            KeyCode::Char('q') => Some(()),
            KeyCode::Char('j') => {
                self.select(KeyCode::Down);
                None
            }
            KeyCode::Char('k') => {
                self.select(KeyCode::Up);
                None
            }
            KeyCode::Enter | KeyCode::Char('i') | KeyCode::Char('a') => {
                self.input_mode = InputMode::Editing;
                self.cursorpos = match self.selected {
                    Widget::Account => self.account.len(),
                    Widget::Password => self.password.len(),
                    _ => 0,
                } as u16;
                None
            }
            _ => None,
        }
    }

    // Some() for break the loop and return, None for continue.
    fn handle_editing(&mut self, key: KeyCode) -> Option<(String, String, u16, String)> {
        match key {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                None
            }
            KeyCode::Enter => match self.selected {
                Widget::Account => {
                    self.select(KeyCode::Down);
                    self.cursorpos = self.password.len() as u16;
                    None
                }
                Widget::Password => {
                    self.select(KeyCode::Down);
                    None
                }
                Widget::Mileage => {
                    self.select(KeyCode::Down);
                    self.cursorpos = self.time.len() as u16;
                    None
                }
                Widget::Time => {
                    self.input_mode = InputMode::Normal;
                    Some((
                        self.account.clone(),
                        self.password.clone(),
                        self.mileage_percent,
                        self.time.clone(),
                    ))
                }
            },
            KeyCode::Tab => match self.selected {
                Widget::Account => {
                    self.select(KeyCode::Down);
                    self.cursorpos = self.password.len() as u16;
                    None
                }
                Widget::Password => {
                    self.select(KeyCode::Down);
                    None
                }
                Widget::Time => {
                    self.select(KeyCode::Down);
                    None
                }
                _ => None,
            },
            KeyCode::Backspace => match self.selected {
                Widget::Account => {
                    if self.cursorpos > 0 {
                        self.cursorpos -= 1;
                        self.account.remove(self.cursorpos as usize);
                    }
                    None
                }
                Widget::Password => {
                    if self.cursorpos > 0 {
                        self.cursorpos -= 1;
                        self.password.pop();
                    }
                    None
                }
                Widget::Time => {
                    if self.cursorpos > 0 {
                        self.cursorpos -= 1;
                        self.time.remove(self.cursorpos as usize);
                    }
                    None
                }
                _ => None,
            },
            KeyCode::Char(c) => match self.selected {
                Widget::Account => {
                    self.account.insert(self.cursorpos as usize, c);
                    self.cursorpos += 1;
                    None
                }
                Widget::Password => {
                    self.password.insert(self.cursorpos as usize, c);
                    self.cursorpos += 1;
                    None
                }
                Widget::Mileage => match c {
                    'h' => {
                        if self.mileage_percent > 0 {
                            self.mileage_percent -= 1;
                        }
                        None
                    }
                    'l' => {
                        if self.mileage_percent < 100 {
                            self.mileage_percent += 1;
                        }
                        None
                    }
                    _ => None,
                },
                Widget::Time => {
                    self.time.insert(self.cursorpos as usize, c);
                    self.cursorpos += 1;
                    None
                }
            },
            KeyCode::Left => match self.selected {
                Widget::Mileage => {
                    if self.mileage_percent > 0 {
                        self.mileage_percent -= 1;
                    }
                    None
                }
                Widget::Account | Widget::Password | Widget::Time => {
                    if self.cursorpos > 0 {
                        self.cursorpos -= 1;
                    }
                    None
                }
            },
            KeyCode::Right => match self.selected {
                Widget::Mileage => {
                    if self.mileage_percent < 100 {
                        self.mileage_percent += 1;
                    }
                    None
                }
                Widget::Account | Widget::Password | Widget::Time => {
                    if self.cursorpos
                        < match self.selected {
                            Widget::Account => self.account.len(),
                            Widget::Password => self.password.len(),
                            Widget::Time => self.time.len(),
                            _ => 0,
                        } as u16
                    {
                        self.cursorpos += 1;
                    }
                    None
                }
            },
            _ => None,
        }
    }

    fn select(&mut self, direction: KeyCode) {
        match self.selected {
            Widget::Account => {
                if let KeyCode::Down = direction {
                    self.selected = Widget::Password;
                }
            }

            Widget::Password => match direction {
                KeyCode::Up => {
                    self.selected = Widget::Account;
                }
                KeyCode::Down => {
                    self.selected = Widget::Mileage;
                }
                _ => {}
            },

            Widget::Mileage => match direction {
                KeyCode::Up => {
                    self.selected = Widget::Password;
                }
                KeyCode::Down => {
                    self.selected = Widget::Time;
                }
                _ => {}
            },

            Widget::Time => {
                if let KeyCode::Up = direction {
                    self.selected = Widget::Mileage;
                }
            }
        }
    }

    fn ui_main(&self, frame: &mut Frame<B>) -> PrettyTuiResult<()> {
        let chunks = Layout::default()
            .margin(2)
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(11),
                    Constraint::Length(12),
                    Constraint::Max(u16::MAX),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let mut text = self.logger.get_message();
        let len = text.len();
        let chunks_height = if chunks[2].height as i16 - 2 >= 0 {
            chunks[2].height as usize - 2
        } else {
            0
        };

        let start_index = if len > chunks_height {
            len - chunks_height
        } else {
            0
        };
        text = text[start_index..].to_vec();
        let log = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("log"));

        frame.render_widget(log, chunks[2]);

        let mut help = match self.input_mode {
            InputMode::Normal => vec![
                Spans::from(vec![
                    Span::styled("<Esc>, q: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("quit"),
                ]),
                Spans::from(vec![
                    Span::styled("<Up>, k: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("up"),
                ]),
                Spans::from(vec![
                    Span::styled("<Down>, j: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("down"),
                ]),
                Spans::from(vec![
                    Span::styled(
                        "<Enter>, i, a: ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("edit mode"),
                ]),
            ],
            InputMode::Editing => vec![
                Spans::from(vec![
                    Span::styled("<Esc>: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("normal mode"),
                ]),
                Spans::from(vec![
                    Span::styled(
                        "<Cr> (Also named Enter): ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("confirm"),
                ]),
            ],
        };

        if let Widget::Mileage = self.selected {
            if let InputMode::Editing = self.input_mode {
                help.push(Spans::from(vec![
                    Span::styled("<Left>, h: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("reduce"),
                ]));
                help.push(Spans::from(vec![
                    Span::styled(
                        "<Right>, l: ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw("increase"),
                ]));
            }
        };

        {
            let chunks = Layout::default()
                .margin(2)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Max(u16::MAX), Constraint::Length(96)].as_ref())
                .split(chunks[0]);
            let help = Paragraph::new(help);
            frame.render_widget(help, chunks[0]);
            let title = Paragraph::new(TITLE2);
            frame.render_widget(title, chunks[1]);
        }
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Max(u16::MAX),
            ])
            .split(chunks[1]);

        let border_selected = Block::default()
            .style(Style::default().fg(Color::Blue))
            .borders(Borders::ALL);
        let border_editing = Block::default()
            .style(Style::default().fg(Color::Yellow))
            .borders(Borders::ALL);

        let account = Paragraph::new(self.account.clone()).block(match self.selected {
            Widget::Account => match self.input_mode {
                InputMode::Editing => border_editing.clone().title("account"),
                InputMode::Normal => border_selected.clone().title("account"),
            },
            _ => Block::default().title("account").borders(Borders::ALL),
        });

        let password = Paragraph::new(Spans::from(vec![Span::raw("*"); self.password.len()]))
            .block(match self.selected {
                Widget::Password => match self.input_mode {
                    InputMode::Editing => border_editing.clone().title("password"),
                    InputMode::Normal => border_selected.clone().title("password"),
                },
                _ => Block::default().title("password").borders(Borders::ALL),
            });
        let mileage = Gauge::default()
            .percent(self.mileage_percent)
            .block(match self.selected {
                Widget::Mileage => match self.input_mode {
                    InputMode::Editing => border_editing.clone().title("mileage"),
                    InputMode::Normal => border_selected.clone().title("mileage"),
                },
                _ => Block::default().title("mileage").borders(Borders::ALL),
            })
            .gauge_style(Style::default().fg(Color::White))
            .label(Span::styled(
                format!("{} %", self.mileage_percent as f64),
                Style::default().fg(Color::Yellow),
            ));

        let time = Paragraph::new(self.time.clone()).block(match self.selected {
            Widget::Time => match self.input_mode {
                InputMode::Editing => border_editing.clone().title("Format: %Y-%m-%d %H:%M:%S"),
                InputMode::Normal => border_selected.clone().title("time"),
            },
            _ => Block::default().title("time").borders(Borders::ALL),
        });

        frame.render_widget(account, chunks[0]);
        frame.render_widget(password, chunks[1]);
        frame.render_widget(mileage, chunks[2]);
        frame.render_widget(time, chunks[3]);

        if let InputMode::Editing = self.input_mode {
            match self.selected {
                Widget::Account => {
                    frame.set_cursor(chunks[0].x + self.cursorpos + 1, chunks[0].y + 1)
                }
                Widget::Password => {
                    frame.set_cursor(chunks[1].x + self.cursorpos + 1, chunks[1].y + 1)
                }
                Widget::Time => frame.set_cursor(chunks[3].x + self.cursorpos + 1, chunks[3].y + 1),
                _ => {}
            }
        }

        Ok(())
    }
}
