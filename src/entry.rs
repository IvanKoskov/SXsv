use std::env;
use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, text::Text, widgets::Paragraph};

use crate::{info_menu::{run_browse, run_help, run_info, run_new}, messages::Message};

const USAGE: &str = "SXsv usage:
  SXsv browse - global file manager
  SXsv new FILE_NAME_WITH_EXTENSION - create a new file
  SXsv info - information about current build
  SXsv help - show this message";

pub fn arguments_sxsv(mut terminal: DefaultTerminal) -> Result<()> {

   

    let args: Vec<String> = env::args().collect();
   // println!("{:?}", args);
    let result: Message;
    if args.len() <= 1 {
      result = Message::ERROR("Unrecognized command...".to_string())
    } else {
      result = Message::SUCCESS("loading...".to_string())
    }

    match result {
        Message::SUCCESS(value) => {
            //println!("{}", value);
            parse_args_run(&args, &mut terminal)?;

           // println!("");
        }
        Message::ERROR(value) => {
           // println!("{}", value);
            
           // println!("{}", USAGE);
        }
        Message::VOID => unreachable!("VOID case should not occur"),
    }

    Ok(())
}

pub fn parse_args_run(args: &[String], terminal: &mut DefaultTerminal) -> Result<()> {


    if args[1] == "info" {
    run_info(terminal)
} else if args[1] == "help" {
    //println!("{:?}", USAGE);
    run_help(terminal);
    Ok(())
} else if args[1] == "browse" {
    run_browse(terminal)
} else if args[1] == "new" {
    if args.len() < 3 {
        println!("Error: File name required for 'new' command");
        println!("\n{}", USAGE);
        Ok(())
    } else {
        run_new(&args[2], terminal)
    }
} else {
      run_help(terminal);
   // println!("Error: Unknown command '{}'", args[1]);
    //println!("\n{}", USAGE);
    Ok(())
}
}


