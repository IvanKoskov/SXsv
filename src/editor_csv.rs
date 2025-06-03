use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode};
use eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, text::Text, widgets::{Block, Borders, List, ListState, Paragraph, Row, Table, TableState}, DefaultTerminal, Frame
};

use crate::read_file::file_read_csv;

pub fn run_csv_editor(terminal: &mut DefaultTerminal, filename: String) -> Result<()> {
    let mut list_action_option = ListState::default();
    let mut popover = false;

    let mut table_action_scroll = TableState::default();

    // filename for use in the closure
    let filename_ref = &filename;

    loop {
        let _ = terminal.draw(|frame: &mut Frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(frame.area());

            let action_list = List::new([
                Text::styled("Sort", Style::new().red()),
                Text::styled("Search", Style::new().red()),
                Text::styled("Info", Style::new().red()),
            ])
            .block(Block::bordered().title("Actions"))
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>");

            // Define the main paragraph
            let paragraph = Paragraph::new(Text::raw("Hello, world!"));

            // Define the popover paragraph
            let parpopover = Paragraph::new(Text::raw("Current mode is CSV"))
                .block(Block::bordered().title("Warning"))
                .style(Style::new().white().on_black()); // Optional: style for visibility

            // Render the main paragraph in layout[0]
          //  frame.render_widget(paragraph, layout[0]);

            // Render the action list in layout[1]
            frame.render_stateful_widget(action_list, layout[1], &mut list_action_option);


/* 
let rows = vec![
    Row::new(vec!["Row11", "Row12", "Row13"]),
    Row::new(vec!["Row21", "Row22", "Row23"]),
    Row::new(vec!["Row31", "Row32", "Row33"]),
];
*/

let rows: Vec<Row<'static>> = file_read_csv(&filename);

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
        Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::new().bold())
            // To add space between the header and the rest of the rows, specify the margin
            .bottom_margin(1),
    )
    .block(Block::bordered().title(filename.as_str()))
    // The selected row, column, cell and its content can also be styled.
    .row_highlight_style(Style::new().reversed())
    // The selected row, column, cell and its content can also be styled.
    .row_highlight_style(Style::new().reversed())
    .column_highlight_style(Style::new().red())
    .cell_highlight_style(Style::new().blue())
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>");

             frame.render_stateful_widget(table, layout[0], &mut table_action_scroll);

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
                                2
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
                            if i >= 2 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    table_action_scroll.select(Some(i));
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
