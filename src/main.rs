mod OS;
mod editor_csv;
mod entry;
mod file;
mod info_menu;
mod messages;
mod time;
use crate::entry::arguments_sxsv;
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
}
