use std::env;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{style::{Style, Stylize}, text::{Line, Text}, widgets::{block, Block, BorderType, Borders, Paragraph, Wrap}, DefaultTerminal, Frame};

use crate::file::File_sxsv;




pub fn run_info(terminal: &mut DefaultTerminal) -> Result<()> {


    loop {
        terminal.draw(|frame: &mut Frame| {
            let b = Block::default()
    .title(Line::from("Build 05312025 (1.0.0)").left_aligned().style(Style::new().red()))
    .title(Line::from("SXSV - fast and neat data formats viewer and editor").centered().bold())
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded);
      
    let body  = vec![Line::from("Platform: "), Line::from("Time installed:")];
    let par = Paragraph::new(body).block(b.clone())
                .wrap(Wrap { trim: true });

frame.render_widget(par, frame.area());

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
