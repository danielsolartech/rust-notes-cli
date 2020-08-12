use crate::notes;
use crate::settings;

/// ### Command
/// Show the help command information in the console.
pub fn help() {
  println!("Command                                    Description");
  println!("");
  println!("create                                     Create a new note.");
  println!("read (note_name)                           Read a note if it exists.");
  println!("edit (note_name)                           Edit an existing note.");
  println!("delete | remove | rm (note_name)           Remove a note if it exists.");
  println!("list | ls                                  View all existing notes.");
  println!("clear | cls                                Clear the console.");
  println!("quit | exit                                Stop the notes app.");
}

/// ### Command
/// Create a new note reading the next lines to put the
/// note name, title and content.
pub fn create_note(notes_dir: &String) {
  let note_name = settings::read_line("Enter the note name: ", &mut settings::create_editor());
  let note_title = settings::read_line("Enter the note title: ", &mut settings::create_editor());
  let note_content = settings::read_line("Enter the note content: ", &mut settings::create_editor());

  if notes::new(notes_dir, &note_name, note_title, note_content) == true {
    println!("The note '{}' was created.", note_name);
  }
}

/// ### Command
/// Read a note by its name.
pub fn read_note(notes_dir: &String, note_name: String) {
  let mut note_name = note_name;
  if note_name.is_empty() == true {
    note_name = settings::read_line("Enter the note name: ", &mut settings::create_editor());
  }

  let note = notes::get(notes_dir, &note_name.to_string());
  if note.title.is_empty() == false {
    println!("\r\n{}\r\n{}\r\n", note.title, note.content);
  }
}

/// ### Command
/// Edit a note by its name.
pub fn edit_note(notes_dir: &String, note_name: String) {
  let mut note_name = note_name;
  if note_name.is_empty() == true {
    note_name = settings::read_line("Enter the note name: ", &mut settings::create_editor());
  }

  let note = notes::get(notes_dir, &note_name.to_string());
  if note.title.is_empty() == false {
    println!("\r\n{}\r\n{}\r\n", note.title, note.content);

    let note_title = settings::read_line("Enter the new note title: ", &mut settings::create_editor());
    let note_content = settings::read_line("Enter the new note content: ", &mut settings::create_editor());
    
    if notes::delete(notes_dir, &note_name.to_string()) == true {
      if notes::new(notes_dir, &note_name, note_title, note_content) == true {
        println!("The note '{}' was edit.", note_name);
      }
    }
  }
}

/// ### Command
/// Delete a note by its name.
pub fn delete_note(notes_dir: &String, note_name: String) {
  let mut note_name = note_name;
  if note_name.is_empty() == true {
    note_name = settings::read_line("Enter the note name: ", &mut settings::create_editor());
  }

  if notes::delete(notes_dir, &note_name.to_string()) == true {
    println!("The note '{}' was deleted.", note_name);
  }
}

/// ### Command
/// View all notes in the current notes directory.
pub fn view_all_notes(notes_path: &std::path::Path) {
  let paths = std::fs::read_dir(notes_path).unwrap();
  let mut index: i32 = 1;

  for path in paths {
    let entry = path.unwrap();
    let path = entry.path();

    // Ignore the path if not a file.
    if path.is_file() == false {
      continue;
    }

    // Get the file name.
    let file_display = path.display();
    let file_name = file_display.to_string();
    let file_name = file_name.as_str();

    // Check if the file name ends in .json
    if file_name.ends_with(".json") == false {
      continue;
    }

    // Get the note name.
    let file_name = file_name
      .replace(".json", "")
      .replace(notes_path.display().to_string().as_str(), "");

    // Show the current note name in the console.
    println!("{}. {}", index, file_name);

    index = index + 1;
  }
}
