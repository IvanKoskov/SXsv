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
use OS::{create_sxsv_files_folder_os, log_os, sxsv_setup};
use std::path::PathBuf;


fn main() -> Result<()> { 
  
    sxsv_setup();

    color_eyre::install()?;
    let terminal = ratatui::init();
    
    // Run arguments_sxsv and ensure terminal restoration
    let result = arguments_sxsv(terminal);
    ratatui::restore();
    
    result // Propagate any error from arguments_sxsv
}



