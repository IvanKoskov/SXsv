use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, text::Text, widgets::Paragraph};
use std::env;

use crate::{
    editor_csv::run_csv_editor,
    info_menu::{run_browse, run_help, run_info, run_new},
    messages::Message,
};

const USAGE: &str = "SXsv usage:
  SXsv browse - global file manager
  SXsv new FILE_NAME_WITH_EXTENSION - create a new file
  SXsv info - information about current build
  SXsv help - show this message";

pub fn arguments_sxsv(mut terminal: DefaultTerminal) -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let result: Message;
    if args.len() <= 1 {
        result = Message::ERROR("Unrecognized command...".to_string())
    } else {
        result = Message::SUCCESS("loading...".to_string())
    }

    match result {
        Message::SUCCESS(value) => {
            parse_args_run(&args, &mut terminal)?;
        }
        Message::ERROR(value) => {}
        Message::VOID => unreachable!("VOID case should not occur"),
    }

    Ok(())
}

pub fn parse_args_run(args: &[String], terminal: &mut DefaultTerminal) -> Result<()> {
    if args[1] == "info" {
        run_info(terminal)
    } else if args[1] == "help" {
        run_help(terminal);
        Ok(())
    } else if args[1] == "browse" {
        run_browse(terminal)
    } else if args[1] == "new" {
        if args.len() < 3 {
            println!("Error: File name required for 'new' command");
            run_help(terminal);
            Ok(())
        } else {
            run_csv_editor(terminal, args[2].clone());
            Ok(())
        }
    } else {
        run_help(terminal);
        Ok(())
    }
}
