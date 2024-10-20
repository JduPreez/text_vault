use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::sync::Mutex;
use std::io::Error;

pub fn read_file(file_path: &str, create_on_not_exists: bool) -> Result<String, Error> {
  let file_mutex = Mutex::new(());
  {
    let _lock = file_mutex.lock().unwrap();
    match OpenOptions::new()
      .read(true)
      .write(create_on_not_exists)
      .create(create_on_not_exists)
      .open(file_path) {
        Ok(mut file) => {
          let mut contents = String::new();
          file.read_to_string(&mut contents);
          return Ok(contents);
        },
        Err(error) => {
          return Err(error);
        }
      }
  };
}


//   let contents = fs::read_to_string(file)
//     .expect(&format!("Failed to read the file {file}"));
  
//   contents
// }

// pub fn maybe_create_file(file: &str) -> String {
//   if (!fs::exists(file)) {
//     fs::write(file, "");
//   }
// }

/*
use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::sync::Mutex;

fn main() {
    let file_path = "example.txt";
    let file_mutex = Mutex::new(());

    // Try to open the file, and if it doesn't exist, create it
    let mut contents = String::new();
    {
        let _lock = file_mutex.lock().unwrap();
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path) {
            Ok(mut file) => {
                if let Err(error) = file.read_to_string(&mut contents) {
                    eprintln!("Error reading file: {}", error);
                }
            }
            Err(error) => {
                eprintln!("Error opening/creating file: {}", error);
                return;
            }
        }
    }

    println!("File contents: {}", contents);
}



*/