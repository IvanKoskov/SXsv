use std::{env, fs::read_to_string};
use chrono::Utc;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use homedir::my_home;
use ratatui::{layout::{Alignment, Constraint, Direction, Layout}, style::{Style, Stylize}, text::{Line, Text}, widgets::{block, Block, BorderType, Borders, List, ListDirection, ListState, Padding, Paragraph, Wrap}, DefaultTerminal, Frame};

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

pub fn run_help(terminal: &mut DefaultTerminal) -> Result<()> {
    let options = [
        "SXsv browse - global file manager",
        "SXsv new FILE_NAME_WITH_EXTENSION - create a new file (only supported formats)",
        "SXsv info - information about current build",
        "SXsv help - show this message",
        "Supported formats - CSV, TSV, JSON, Parquet and more in the future",
    ];

    let mut state = ListState::default();
    state.select(Some(0)); // Start with first item selected

    loop {
        terminal.draw(|frame: &mut Frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.area());

            let list = List::new(options)
                .block(Block::bordered().title("SXsv USAGE").red())
                .style(Style::new().white())
                .highlight_style(Style::new().italic().red())
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, chunks[0], &mut state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Down => {
                    let i = match state.selected() {
                        Some(i) => if i >= options.len() - 1 { 0 } else { i + 1 },
                        None => 0,
                    };
                    state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match state.selected() {
                        Some(i) => if i == 0 { options.len() - 1 } else { i - 1 },
                        None => 0,
                    };
                    state.select(Some(i));
                }
                _ => {}
            }
        }
    }

    Ok(())
}
