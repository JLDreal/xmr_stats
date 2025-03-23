mod stats;
use stats::Stats;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use std::error::Error;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
fn main() -> Result<(), Box<dyn Error>> {
    // Clear the terminal screen
    execute!(std::io::stdout(), Clear(ClearType::All))?;

    // Initialize the terminal
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Create and update stats
    let mut stats = Stats::new();
    stats.update()?;

    // Render the stats inside a bordered block
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Monero Stats")
            .title_style(Style::default().fg(Color::Green));
        let paragraph = Paragraph::new(stats.to_spans()).block(block);
        f.render_widget(paragraph, size);
    })?;

    Ok(())
}