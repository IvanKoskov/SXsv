use std::env;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, text::Text, widgets::Paragraph};

pub fn run_info(terminal: &mut DefaultTerminal) -> Result<()> {
   

    loop {

         terminal.draw(|frame: &mut Frame| {
        let text = Text::raw("SXsv Build Info\nVersion: 0.1.0\nAuthor: Unknown");
        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, frame.area());
    })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}


pub fn run_browse(_terminal: &mut DefaultTerminal) -> Result<()> {
    // Placeholder for file manager logic
    println!("Browse mode not implemented yet");
    Ok(())
}

pub fn run_new(_filename: &str, _terminal: &mut DefaultTerminal) -> Result<()> {
    // Placeholder for file creation logic
    println!("File creation not implemented yet");
    Ok(())
}
