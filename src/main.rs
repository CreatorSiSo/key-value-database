use std::collections::HashMap;
use std::fs;

fn main() {
  let mut arguments = std::env::args().skip(1);
  let database_name = arguments
    .next()
    .expect("No database to be edited was provided!");

  let mut database = Database::new(&database_name)
    .expect(format!("Could not create database '{database_name}'").as_str());

  for argument in arguments {
    if let Some((key, value)) = string_to_key_value_pair(&argument, ":") {
      database.data.insert(key, value);
    }
  }

  std::process::exit(match database.save() {
    Ok(()) => {
      println!("Saved database successfully.");
      0
    }
    Err(error) => {
      eprintln!("Could not save database '{}': {}", database.name, error);
      1
    }
  });
}

#[derive(Default, Debug, Clone)]
struct Database {
  name: String,
  data: HashMap<String, String>,
}

impl Database {
  fn new(name: &str) -> Result<Database, std::io::Error> {
    let file_content = fs::read_to_string(format!("{}.db", name))?;

    // Convert String into a HashMap
    let mut data = HashMap::default();
    for line in file_content.lines() {
      if let Some((key, value)) = string_to_key_value_pair(line, "\t") {
        data.insert(key, value);
      }
    }

    Ok(Database {
      name: String::from(name),
      data,
    })
  }

  fn save(&self) -> Result<(), std::io::Error> {
    let mut content = String::default();

    // Convert HashMap into a String
    for (key, value) in &self.data {
      content.push_str(format!("{}\t{}\n", key, value).as_str());
    }

    fs::write(format!("{}.db", self.name), content)
  }
}

fn string_to_key_value_pair(string: &str, seperator: &str) -> Option<(String, String)> {
  if let Some((key, value)) = string.split_once(seperator) {
    if key.trim().is_empty() {
      return None;
    }
    return Some((key.to_owned(), value.to_owned()));
  }

  None
}
