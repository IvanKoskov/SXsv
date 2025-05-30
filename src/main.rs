mod entry;
mod file;
mod OS;
mod messages;
mod info_menu;
use crate::entry::arguments_sxsv;
use crate::messages::Message;
use color_eyre::Result;
use crossterm::event::Event;
use entry::parse_args_run;
use ratatui::{crossterm::terminal, DefaultTerminal, Frame};
use homedir::my_home;
use OS::log_os;
use std::path::PathBuf;


fn main() -> Result<()> { 

    let value_os = log_os();

    println!("{:?}", value_os);
    
    //println!("Current OS: {}", std::env::consts::OS);

    color_eyre::install()?;
    let terminal = ratatui::init();
    
    // Run arguments_sxsv and ensure terminal restoration
    let result = arguments_sxsv(terminal);
    ratatui::restore();
    
    result // Propagate any error from arguments_sxsv
}



