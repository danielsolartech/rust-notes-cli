use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
  pub title: String,
  pub content: String,
}

/// Get the note name in the command params.
pub fn get_note_name(next_param: std::option::Option<&str>) -> String {
  if next_param.is_none() == false {
    next_param.unwrap().to_string()
  } else {
    String::new()
  }
}

/// Get the note path.
/// 
/// ## Example
/// ```
/// use crate::notes;
/// 
/// let path = notes::get_note_path(&String::from("/home/.notes/"), &String::from("example"));
/// // /home/.notes/example.json
/// ```
pub fn get_note_path(notes_dir: &String, note_name: &String) -> String {
  format!("{}{}.json", notes_dir, note_name)
}

#[test]
fn get_note_path_test() {
  let path = get_note_path(&String::from("/home/.notes/"), &String::from("example"));
  assert_eq!(path, String::from("/home/.notes/example.json"));
}

/// Check if the note path exists.
/// 
/// ## Example
/// ```
/// use crate::notes;
/// use crate::settings;
/// 
/// let exist = notes::has(&get_note_path(&settings::get_notes_dir(), &String::from("example")));
/// // false
/// ```
pub fn has(note_path: &String) -> bool {
  let path = Path::new(note_path);
  path.is_file() && path.exists()
}

#[test]
fn has_test() {
  let exist = has(&get_note_path(&crate::settings::get_notes_dir(), &String::from("example")));
  assert_eq!(exist, false);
}

/// Create a new note with its name, title and content.
/// 
/// ## Example
/// ```
/// use crate::notes;
/// use crate::settings;
/// 
/// let new_note = notes::new(
///   &settings::get_notes_dir(),
///   &String::from("example"),
///   String::from("Note title"),
///   String::from("Note content")
/// );
/// // true
/// ```
pub fn new(
  notes_dir: &String,
  note_name: &String,
  note_title: String,
  note_content: String,
) -> bool {
  let note_path = &get_note_path(notes_dir, note_name);
  if has(note_path) == true {
    println!("The note name '{}' already exists.", note_name);
    return false;
  }

  if note_title.is_empty() == true {
    println!("The note title is empty.");
    return false;
  }

  if note_content.is_empty() == true {
    println!("The note content is empty.");
    return false;
  }

  let note = Note {
    title: note_title,
    content: note_content,
  };

  let file_content = serde_json::to_string(&note).unwrap();

  let mut note_file = fs::File::create(note_path).expect("Unable to create the note file.");

  note_file
    .write_all(file_content.as_bytes())
    .expect("Unable to write the note file.");

  has(note_path)
}

#[test]
fn new_test() {
  let new_note = new(
    &crate::settings::get_notes_dir(),
    &String::from("example"),
    String::from("Note title"),
    String::from("Note content")
  );

  assert_eq!(new_note, true);
}

/// Get the note data from a file in the notes directory.
/// 
/// ## Example
/// ```
/// use crate::notes;
/// use crate::settings;
/// 
/// let note = notes::get(&settings::get_notes_dir(), &String::from("example"));
/// // { "title": "Note title", "content": "Note content" }
/// ```
pub fn get(notes_dir: &String, note_name: &String) -> Note {
  let note_path = get_note_path(notes_dir, note_name);
  if has(&note_path) == false {
    println!("The note name '{}' does not exist.", note_name);
    return Note {
      title: String::new(),
      content: String::new(),
    };
  }

  let file_content = fs::read_to_string(note_path).expect("The note file does not exist.");
  serde_json::from_str(&file_content).unwrap()
}

#[test]
fn get_test() {
  let note = get(&crate::settings::get_notes_dir(), &String::from("example"));
  let note_to_compare = Note {
    title: String::from("Note title"),
    content: String::from("Note content")
  };

  assert_eq!(
    serde_json::to_string(&note).unwrap(),
    serde_json::to_string(&note_to_compare).unwrap()
  );
}

/// Delete a note file from notes directory.
/// 
/// ## Example
/// ```
/// use crate::notes;
/// use crate::settings;
/// 
/// let deleted = notes::delete(&settings::get_notes_dir(), &String::from("example"));
/// // true
/// ```
pub fn delete(notes_dir: &String, note_name: &String) -> bool {
  let note_path = &get_note_path(notes_dir, note_name);
  if has(note_path) == false {
    println!("The note name '{}' does not exist.", note_name);
    return false;
  }

  fs::remove_file(note_path).is_ok() && !has(note_path)
}

#[test]
fn delete_test() {
  let deleted = delete(&crate::settings::get_notes_dir(), &String::from("example"));
  assert_eq!(deleted, true);
}
