// ╔══════════════════════════════════════════════════════╗
// ║                     IMPORTS                          ║
// ╚══════════════════════════════════════════════════════╝


use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;



// ╔==========================================================╗
// ║                     HELPER FUNCTIONS                     ║
// ╚==========================================================╝

#[derive(Debug)]
struct Snake {
    body: Vec<(u16, u16)>,
    direction: direction,
    food: (u16, u16),
    score: u32,
}


#[derive(Debug, PartialEq, Eq)]
enum direction{
    Up,
    Down,
    Left,
    Right,
}



impl Default for Snake {

    fn default() -> Self {
        Snake {
            body: vec![(0, 0)],
            direction: direction::Right,
            food: (0, 0),
            score: 0,
        }
    }

}


fn init_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}



// ╔════════════════════════════════════════════════════════════╗
// ║                     MAIN FUNCTION                          ║
// ╚════════════════════════════════════════════════════════════╝

fn main() {
    
}
