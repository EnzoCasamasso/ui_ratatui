use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use ratatui::{prelude::*, widgets::*, Frame};
use std::io;

#[derive(Debug)]
pub struct CreateView;

impl CreateView {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            Clear(ClearType::All),
            crossterm::terminal::EnterAlternateScreen
        )?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        //Main loop
        'mainloop: loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break 'mainloop,
                    _ => {}
                }
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            Clear(ClearType::All)
        )?;
        Ok(())
    }

    pub fn ui(&mut self, frame: &mut Frame) {
        let size = frame.size();
        frame.render_widget(
            Paragraph::new("Hello world")
                .block(Block::default().title("Teste").borders(Borders::ALL)),
            size,
        );
    }
}
