pub mod commands;
pub mod notes;
pub use crate::notes::*;
pub mod settings;
pub use crate::settings::*;

use std::fs;
use std::path::Path;

pub fn main() {
  // Get the notes directory from environment var.
  let notes_dir = &settings::get_notes_dir();
  let path = Path::new(notes_dir);

  // Check if the path is a file
  if path.is_file() == true {
    println!("The dir path is not valid.");
    return;
  } else if path.exists() == false {
    // If the path does not exist, create it.
    let created_dir = fs::create_dir_all(notes_dir);
    if created_dir.is_err() == true {
      println!("Error creating directory: {}", created_dir.unwrap_err());
      return;
    }
  }

  // Show the initial message on the console.
  settings::initial_message();

  // Create a new rustyline editor.
  let mut editor = settings::create_editor();

  loop {
    // Read the current line of the console and split it in whitespaces.
    let line = settings::read_line("> ", &mut editor);
    let mut params = line.split_whitespace();

    // Get the first parameter of the readed line.
    let command = params.next();

    // Check if the first parameter is a command and execute a task.
    match command.unwrap() {
      "help" => commands::help(),
      "create" => commands::create_note(notes_dir),
      "read" => commands::read_note(notes_dir, notes::get_note_name(params.next())),
      "edit" => commands::edit_note(notes_dir, notes::get_note_name(params.next())),
      "delete" | "remove" | "rm" => commands::delete_note(notes_dir, notes::get_note_name(params.next())),
      "list" | "ls" => commands::view_all_notes(path),
      "clear" | "cls" => {
        // Clear the console and then send the initial message.
        std::process::Command::new("clear").status().unwrap();
        settings::initial_message();
      }
      "exit" | "quit" => {
        println!("Bye!");
        break;
      }
      _ => {}
    }
  }
}
