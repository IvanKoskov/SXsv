use color_eyre::{eyre, owo_colors::{colors::css::Red, OwoColorize}};
use crossterm::{event::{self, Event, KeyCode}, style::Color};
use eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, symbols::line::VERTICAL, text::Text, widgets::{Block, Borders, List, ListState, Paragraph, Row, Table, TableState, Wrap}, DefaultTerminal, Frame
};

use crate::read_file::{self, file_read_csv, file_read_first_line, file_read_lines};

pub fn run_csv_editor(terminal: &mut DefaultTerminal, filename: String) -> Result<()> {
    let mut highlight_col: Color = Color::DarkRed;
    let mut list_action_option = ListState::default();
    let mut popover = false;

    let mut table_action_scroll = TableState::default();

    // filename for use in the closure
    let filename_ref = &filename;

    let mut rows_len = file_read_lines(&filename);

    let mut editor_size: u16 = 70;
    let mut action_panel_size: u16 = 30;

    loop {
        let _ = terminal.draw(|frame: &mut Frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(editor_size), Constraint::Percentage(action_panel_size)])
                .split(frame.area());

            let editor_inner_tab = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(85), Constraint::Percentage(15)])
            .split(layout[0]);

            let par_help_editor = Paragraph::new("H - scroll editor up, L - scroll down, Actions - arrows").wrap(Wrap { trim: true }).block(Block::bordered().title("Navigation").red());

            frame.render_widget(par_help_editor, editor_inner_tab[1]);
            let action_list = List::new([
                Text::styled("Sort", Style::new().red()),
                Text::styled("Search", Style::new().red()),
                Text::styled("Info", Style::new().red()),
            ])
            .block(Block::bordered().title("Actions"))
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>");

            // Define the main paragraph
            //let paragraph = Paragraph::new(Text::raw("Hello, world!"));

            // Define the popover paragraph
            let parpopover = Paragraph::new(Text::raw("Current mode is CSV"))
                .block(Block::bordered().title("Warning"))
                .style(Style::new().white().on_black()); // Optional: style for visibility

            // Render the main paragraph in layout[0]
          //  frame.render_widget(paragraph, layout[0]);

            // Render the action list in layout[1]
            frame.render_stateful_widget(action_list, layout[1], &mut list_action_option);



/*let rows = vec![
    Row::new(vec!["Row11", "Row12", "Row13"]),
    Row::new(vec!["Row21", "Row22", "Row23"]),
    Row::new(vec!["Row31", "Row32", "Row33"]),
];
*/

let rows = file_read_csv(&filename);

            // Columns widths are constrained in the same way as Layout...
let widths = [
    Constraint::Length(20),
    Constraint::Length(20),
    Constraint::Length(20),
];



let table = Table::new(rows, widths)
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // You can set the style of the entire Table.
    .style(Style::new().blue())
    // It has an optional header, which is simply a Row always visible at the top.
    .header(
        Row::new(file_read_first_line(&filename))
            .style(Style::new().bold())
            // To add space between the header and the rest of the rows, specify the margin
            .bottom_margin(1),
    )
    .block(Block::bordered().title(filename.as_str()))
    // The selected row, column, cell and its content can also be styled.
    .row_highlight_style(Style::new().red())
    // The selected row, column, cell and its content can also be styled.
    .row_highlight_style(Style::new().red().reversed())  //actual row col
    .column_highlight_style(Style::new().red())
    .cell_highlight_style(Style::new().red())
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>");

             frame.render_stateful_widget(table, editor_inner_tab[0], &mut table_action_scroll);

            // Render the popover if active
            /*   if popover {
                // Create a centered popover area (e.g., 30% of terminal width and 5 lines tall)
                let popover_area = centered_rect(30, 20, frame.area());
                frame.render_widget(parpopover, popover_area);
            } */

            if popover {
                // Centered Rect for the popover
                let popover_width = frame.area().width * 30 / 100; // 30% of terminal width
                let popover_height = 10; // Fixed height of 5 lines
                let popover_x = (frame.area().width - popover_width) / 2; // Center horizontally
                let popover_y = (frame.area().height - popover_height) / 2; // Center vertically
                let popover_area = Rect {
                    x: popover_x,
                    y: popover_y,
                    width: popover_width,
                    height: popover_height,
                };
                frame.render_widget(parpopover, popover_area);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Down  => {
                    let i = match list_action_option.selected() {
                        Some(i) => {
                            if i >= 3 - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    list_action_option.select(Some(i));
                }
                KeyCode::Up  => {
                    let i = match list_action_option.selected() {
                        Some(i) => {
                            if i == 0 {
                                3
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    list_action_option.select(Some(i));
                }

                 KeyCode::Char('h')  => {
                    let i = match table_action_scroll.selected() {
                        Some(i) => {
                            if i == 0 {
                                rows_len
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    table_action_scroll.select(Some(i));
                }

                 KeyCode::Char('l')  => {
                    let i = match table_action_scroll.selected() {
                        Some(i) => {
                            if i >= rows_len - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    table_action_scroll.select(Some(i));
                }

                 KeyCode::Right  => {
                  if action_panel_size == 0 || editor_size == 0 {
                  
                  action_panel_size = 30;
                  editor_size = 70;

                  } 
                  else {

                    editor_size = editor_size + 1;
                    action_panel_size = action_panel_size - 1;

                  }
                
                }

                KeyCode::Left  => {
                  if action_panel_size == 0 || editor_size == 0 {
                  
                  action_panel_size = 30;
                  editor_size = 70;

                  } 
                  else {

                    editor_size = editor_size - 1;
                    action_panel_size = action_panel_size + 1;

                  }
                
                }

                KeyCode::Char('m') => {
                    popover = !popover; // Toggle popover
                }
                _ => {}
            }
        }
    }
    Ok(())
}

// Helper to create a centered rectangle for the popover
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
