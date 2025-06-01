use crossterm::terminal;
use ratatui::{crossterm::event::{self, Event, KeyCode}, layout::{Constraint, Direction, Layout}, widgets::Paragraph, DefaultTerminal};

pub fn run_csv_editor(terminal: &mut DefaultTerminal, filename: String) {

loop {

terminal.draw(|frame:&mut ratatui::Frame |{

    let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(frame.area());

    //frame.render_widget(widget, area);

});


if let Ok(Event::Key(key)) = event::read() {
            if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                break;
             }
             
        }


}

}