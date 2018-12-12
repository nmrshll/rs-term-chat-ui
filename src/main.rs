#![feature(uniform_paths)]
#![feature(rustc_private)]
#![feature(fmt_internals)]

extern crate failure;
extern crate termion;
extern crate tui;
extern crate unicode_width;

#[allow(dead_code)]
mod util;

use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, SelectableList, Text, Widget};
use tui::{Frame, Terminal};

use util::event::{Event, Events};

struct App<'a> {
    items: Vec<&'a str>,
    selected: Option<usize>,
    // events: Vec<(&'a str, &'a str)>,
    // info_style: Style,
    //     warning_style: Style,
    //     error_style: Style,
    //     critical_style: Style,
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: Vec<String>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec![
                "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9",
                "Item10", "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17",
                "Item18", "Item19", "Item20", "Item21", "Item22", "Item23", "Item24",
            ],
            selected: None,
            // info_style: Style::default().fg(Color::White),
            // warning_style: Style::default().fg(Color::Yellow),
            // error_style: Style::default().fg(Color::Magenta),
            // critical_style: Style::default().fg(Color::Red),
            input: String::new(),
            messages: Vec::new(),
        }
    }

    // fn advance(&mut self) {
    //     let event = self.events.pop().unwrap();
    //     self.events.insert(0, event);
    // }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.size());

            let style = Style::default().fg(Color::Black).bg(Color::White);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("List"))
                .items(&app.items)
                .select(app.selected)
                .style(style)
                .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::Bold))
                .highlight_symbol(">")
                .render(&mut f, chunks[0]);

            match draw_chatroom(&mut f, &app, chunks[1]) {
                _ => {}
            };
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.selected = None;
                }
                Key::Down => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected >= app.items.len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Up => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(app.items.len() - 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Char('\n') => {
                    app.messages.push(app.input.drain(..).collect());
                }
                Key::Char(c) => {
                    app.input.push(c);
                }
                Key::Backspace => {
                    app.input.pop();
                }
                _ => {}
            },
            Event::Tick => {
                // app.advance();
            }
        }
    }

    Ok(())
}

fn draw_chatroom<B>(f: &mut Frame<B>, app: &App, area: Rect) -> Result<(), failure::Error>
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Min(3)].as_ref())
        .split(area);

    let messages = app
        .messages
        .iter()
        .rev()
        .enumerate()
        .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));
    List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Messages"))
        .start_corner(Corner::BottomLeft)
        .render(f, chunks[0]);

    Paragraph::new([Text::raw(&app.input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .wrap(true)
        .render(f, chunks[1]);

    Ok(())
}
