use color_eyre::eyre;
//use color_eyre::eyre::Ok;
use crossterm::terminal;
use ratatui::{crossterm::event::{self, Event, KeyCode}, layout::{Constraint, Direction, Layout}, style::{Style, Stylize}, symbols::border, text::Text, widgets::{Block, Borders, List, ListState, Paragraph}, DefaultTerminal, Frame};
use eyre::Result;

pub fn run_csv_editor(terminal: &mut DefaultTerminal, filename: String) -> Result<()> {
   /*
   STATES
    */

    let mut list_action_option = ListState::default();


    loop {
        let _ = terminal.draw(|frame: &mut ratatui::Frame| {
          let layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(70),
        Constraint::Percentage(30),
    ])
    .split(frame.area());

    let action_list = List::new([
    Text::styled("Sort", Style::new().red()),
    Text::styled("Search", Style::new().red()),
     Text::styled("Info", Style::new().red()),
]).block(Block::bordered().title("Actions")).highlight_style(Style::new().italic())
    .highlight_symbol(">>");

let paragraph = Paragraph::new(Text::raw("Hello, world!"));

    frame.render_widget(paragraph, layout[0]);
    frame.render_stateful_widget(action_list, layout[1],&mut list_action_option);
    
    

        });

           if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Down => {
                    let i = match list_action_option.selected() {
                        Some(i) => if i >= 2 - 1 { 0 } else { i + 1 },
                        None => 0,
                    };
                    list_action_option.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match list_action_option.selected() {
                        Some(i) => if i == 0 { 2 - 1 } else { i - 1 },
                        None => 0,
                    };
                    list_action_option.select(Some(i));
                }
                _ => {}
            }
        }
    }
    Ok(())
}