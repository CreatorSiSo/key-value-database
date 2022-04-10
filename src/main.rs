use std::collections::HashMap;
use std::fs;

fn main() {
  let arguments = std::env::args().skip(1);
  let mut database = Database::new("👀").expect("Could not read database file '👀.db'");

  arguments
    .map(|argument| {
      let (key, value) = argument.split_once(":").unwrap_or_default();
      (key.to_owned(), value.to_owned())
    })
    .for_each(|pair| {
      database.data.insert(pair.0, pair.1);
    });

  std::process::exit(match database.save() {
    Ok(()) => {
      println!("Saved store successfully.");
      0
    }
    Err(error) => {
      eprintln!("Could not save store '{}': {}", database.name, error);
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
    let mut data = HashMap::default();

    for line in file_content.lines() {
      if line.contains("\t") {
        let (key, value) = line.split_once("\t").unwrap_or_default();
        data.insert(String::from(key), String::from(value));
      }
    }

    Ok(Database {
      name: String::from(name),
      data,
    })
  }

  fn save(&self) -> Result<(), std::io::Error> {
    let mut content = String::default();

    for pair in &self.data {
      content.push_str(format!("{}\t{}\n", pair.0, pair.1).as_str());
    }

    fs::write(format!("{}.db", self.name), content)
  }
}
