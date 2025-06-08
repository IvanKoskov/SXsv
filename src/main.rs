mod OS;
mod editor_csv;
mod entry;
mod file;
mod info_menu;
mod messages;
mod read_file;
mod time;
use std::env;

use crate::{entry::arguments_sxsv, read_file::{file_read_csv, file_read_first_line, file_read_lines}};
use csv::Reader;
use OS::sxsv_setup;
use color_eyre::{Result, eyre::Ok};
use time::sxsv_time;

fn main() -> Result<()> {

     sxsv_time();

    sxsv_setup();

    color_eyre::install()?;
    let terminal = ratatui::init();

    // Run arguments_sxsv and ensure terminal restoration
    let result = arguments_sxsv(terminal);

    ratatui::restore();

    result // Propagate any error from arguments_sxsv






//Ok(())
}
