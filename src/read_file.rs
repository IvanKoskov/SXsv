use std::{fs::{read_to_string, File}, io::{BufRead, BufReader, Error, Read}};
use csv::{Reader, ReaderBuilder, WriterBuilder};
use ratatui::{layout::Rows, widgets::Row};

pub fn file_read_lines(name: &String) -> usize {
  
  let f = File::open(name).expect("Failed to open file");
  let mut reader = BufReader::new(f);
  let line_count = reader.lines().count();

  //println!("{}", line_count);
  
  line_count

}



// Store the file contents in a struct to manage lifetime
pub fn file_read_csv(name: &str) -> Vec<Row<'static>> {
    let mut file = File::open(name).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file");

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(file_contents.as_bytes());

    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result.expect("Failed to read CSV record");
        let cells: Vec<&'static str> = record
            .iter()
            .map(|s| Box::leak(s.to_owned().into_boxed_str()) as &'static str)
            .collect();
        rows.push(Row::new(cells));
    }

    rows
}

pub fn file_read_first_line(name: &str) -> Vec<String> {
  let mut rdr = csv::ReaderBuilder::new()
    .delimiter(b';')  // Use semicolon instead of comma
    .from_path(name)
    .expect("Failed to open CSV file");

  let record = rdr.headers().expect("Failed to read headers");
  let vector: Vec<String> = record.iter().map(|s| s.to_string()).collect();
  //println!("{:?}", vector);

  vector
}


