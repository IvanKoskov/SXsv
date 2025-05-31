mod entry;
mod file;
mod OS;
mod messages;
mod info_menu;
use crate::entry::arguments_sxsv;
use color_eyre::{eyre::Ok, Result};
use OS::sxsv_setup;


fn main() -> Result<()> { 
    
sxsv_setup();

    color_eyre::install()?;
    let terminal = ratatui::init();
    
    // Run arguments_sxsv and ensure terminal restoration
    let result = arguments_sxsv(terminal);
     
    ratatui::restore();
    
    result // Propagate any error from arguments_sxsv
   
}



