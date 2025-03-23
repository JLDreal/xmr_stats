mod stats;
use stats::Stats;

use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Clear the terminal screen
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    let mut stats = Stats::new();

    // Update stats from the API
    if let Err(e) = stats.update() {
        eprintln!("Error updating stats: {}", e);
    }

    // Initialize the terminal
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    
    // Render the UI
    terminal.draw(|f| {

        // Create the content as a vector of Lines
        let content = stats.to_spans();
        // Calculate the maximum width of the content
        let max_width = content
            .iter()
            .map(|line| line.width())
            .max()
            .unwrap_or(0);

        // Create a bordered block with a title
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                format!("XMR"),
                Style::default().fg(Color::Rgb(255, 165, 0)),
            ));

        // Create a Paragraph with the content and block
        let paragraph = Paragraph::new(content.clone()).block(block);

        // Define the layout area dynamically based on content width
        let layout_area = Rect {
            x: 0, 
            y: 0,
            width: max_width as u16 + 4, // Add padding for borders
            height: content.len() as u16 + 2, // Add padding for borders
        };

        // Render the paragraph in the calculated area
        f.render_widget(paragraph, layout_area);
    })?;
    println!();
    Ok(())
}