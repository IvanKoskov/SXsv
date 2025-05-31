use std::{env, fs::read_to_string};
use chrono::Utc;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use homedir::my_home;
use ratatui::{layout::Alignment, style::{Style, Stylize}, text::{Line, Text}, widgets::{block, Block, BorderType, Borders, Padding, Paragraph, Wrap}, DefaultTerminal, Frame};

use crate::file::File_sxsv;




pub fn run_info(terminal: &mut DefaultTerminal) -> Result<()> {

     let read_time_when_installed = | | -> String {

           let home = my_home()
    .ok()
    .flatten()
    .map(|p| p.display().to_string())
    .unwrap_or_else(|| "Unknown".to_string());
//println!("{}", home);

       let path_to_time = home + "/.time_sxsv";
      
      let contents = read_to_string(path_to_time)
          .unwrap_or_else(|_| "Unknown".to_string());
    
      contents
      };


    loop {
        terminal.draw(|frame: &mut Frame| {
            let b = Block::default()
    .title(Line::from("Build 05312025 (1.0.0)").left_aligned().style(Style::new().red()))
    .title(Line::from("SXSV - fast and neat data formats viewer and editor").centered().bold())
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded);

    

    let body  = vec![Line::from(format!("Platform: {}", std::env::consts::OS)), Line::from(format!("Installed current version on: {:?}", read_time_when_installed()))
    , Line::from("Main developer and  maintainer: Ivan Koskov"), Line::from("License:  BSD-3-Clause license"), Line::from("Copyright (c) [2025], [Ivan Koskov aka Evan Matthew]")];
let par = Paragraph::new(body).block(b.clone().padding(Padding::new(
0, // left
0, // right
 2, // top
0, // bottom
)))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
            

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
