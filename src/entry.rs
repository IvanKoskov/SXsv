use std::env;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, text::Text, widgets::Paragraph};

use crate::messages::Message;

const USAGE: &str = "SXsv usage:
  SXsv browse - global file manager
  SXsv new FILE_NAME_WITH_EXTENSION - create a new file
  SXsv info - information about current build
  SXsv help - show this message";

pub fn arguments_sxsv(mut terminal: DefaultTerminal) -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut result: Message = if args.len() < 2 {
        Message::ERROR("Unrecognized command...".to_string())
    } else {
        Message::SUCCESS("loading...".to_string())
    };

    match result {
        Message::SUCCESS(value) => {
            println!("{}", value);
            parse_args_run(&args, &mut terminal)?;
        }
        Message::ERROR(value) => {
            println!("{}", value);
            println!("\n{}", USAGE);
        }
        Message::VOID => unreachable!("VOID case should not occur"),
    }

    Ok(())
}

pub fn parse_args_run(args: &[String], terminal: &mut DefaultTerminal) -> Result<()> {
    match args[1].as_str() {
        "info" => run_info(terminal),
        "help" => {
            println!("{}", USAGE);
            Ok(())
        }
        "browse" => run_browse(terminal),
        "new" => {
            if args.len() < 3 {
                println!("Error: File name required for 'new' command");
                println!("\n{}", USAGE);
                Ok(())
            } else {
                run_new(&args[2], terminal)
            }
        }
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            println!("\n{}", USAGE);
            Ok(())
        }
    }
}

fn run_info(terminal: &mut DefaultTerminal) -> Result<()> {
   

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

fn run_browse(_terminal: &mut DefaultTerminal) -> Result<()> {
    // Placeholder for file manager logic
    println!("Browse mode not implemented yet");
    Ok(())
}

fn run_new(_filename: &str, _terminal: &mut DefaultTerminal) -> Result<()> {
    // Placeholder for file creation logic
    println!("File creation not implemented yet");
    Ok(())
}