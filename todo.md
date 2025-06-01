## Todos

1. Create CSV files editor (Edit contents, view of source, some small tools like search, sort)

2. Create editors for other formats (Current goal is CSV editor)

## Steps for CSV editor

Use the the template below and draw stuff using it

```
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

```

### IMPORTANT!

We want to use frame.render_stateful_widget(...) because we will have data to update and keep track of so use it (CSV text basically).

https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html
https://docs.rs/ratatui