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
                    KeyCode::Esc => break 'mainloop,
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
        let label_frame_size = frame.size().to_string();
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());

        // let span = Span::raw("Span widget");
        let line = Line::from(vec!["Vim ".red(), "is ".blue(), "good!".yellow()])
            .alignment(Alignment::Center);

        let paragraph =
            Paragraph::new(line).block(Block::default().title("File_one").borders(Borders::ALL));
        let paragraph_hex = Paragraph::new(format!("Size: {}", label_frame_size))
            .block(Block::default().title("File_two").borders(Borders::ALL));

        //Render layout here:
        frame.render_widget(paragraph, layout[0]);
        frame.render_widget(paragraph_hex, layout[1]);
    }
}
