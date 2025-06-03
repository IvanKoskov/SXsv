use std::{fs::File, io::{BufRead, BufReader}};
use ratatui::widgets::Row;

pub fn file_read_lines(name: &String) -> usize {
  
  let f = File::open(name).expect("Failed to open file");
  let mut reader = BufReader::new(f);
  let line_count = reader.lines().count();

  //println!("{}", line_count);
  
  line_count

}

pub fn file_read<'a>(name: &String) -> Vec<Row<'a>> {

 Vec::new()

}