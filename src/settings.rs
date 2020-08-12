use std::env;

/// Get an environment variable by its name and if it
/// does not exist return a default value.
/// 
/// ## Example:
/// ```
/// let notes_directory = get_config(String::from("NOTES_DIR"), "/home/.notes");
/// ```
fn get_config(name: String, default_return: String) -> String {
  let env_var = env::var(name);

  if env_var.is_err() == true {
    default_return
  } else {
    env_var.unwrap()
  }
}

/// Get the home directory from the environment variables using the dirs package.
/// If it does not exist return an empty string.
pub fn get_home_dir() -> String {
  match dirs::home_dir() {
    Some(path) => path.display().to_string(),
    None => String::new()
  }
}

/// Get the notes directory from the environment variables.
/// if it does not exist return the home directory (Unix) or user profile directory
/// and add "/.notes/" to the end.
pub fn get_notes_dir() -> String {
  get_config(String::from("NOTES_DIR"), get_home_dir() + "/.notes/")
}

/// Show the initial message in the console.
pub fn initial_message() {
  println!("Welcome to Rust Notes CLI by Daniel Solarte");
  println!("If you do not know how to use this, type 'help'.");
  println!("");
}

/// Create a new rustyline editor.
pub fn create_editor() -> rustyline::Editor<()> {
  rustyline::Editor::<()>::new()
}

/// Read the current line using rustyline.
pub fn read_line(message: &str, editor: &mut rustyline::Editor<()>) -> String {
  match editor.readline(message) {
    Ok(line) => {
      editor.add_history_entry(line.as_str());
      line.to_string()
    }
    Err(_) => "exit".to_string(),
  }
}
