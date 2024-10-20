use std::env;
use std::error::Error;
mod file_io;

fn main() {
  let args: Vec<String> = env::args().collect();

  let task = &args[1]; // show or hide
  let file_path = &args[2];

  println!("Task: {task}");
  println!("File: {file_path}");

  match &file_io::read_file(&file_path, true) {
    Ok(file_content) => {
      println!("Content: {file_content}");
    },
    Err(error) => {
      let msg = error.to_string();
      println!("Error: {msg}");

      if let Some(cause) = error.source() {
        println!("Cause: {cause}");
      } 
    }
  }
}
