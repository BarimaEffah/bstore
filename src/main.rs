use std::collections::HashMap;
fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("you didn't provide a key");
    let value = arguments.next().expect("you didn't provide a value");
    println!("the key is {} and the value is {}", key, value);

    let mut database = Database::new().expect("Database crashed");
    database.set(key.to_uppercase(), value.clone());
    database.set(key, value);
}

struct Database {
    map: HashMap<String, String>,
    flushed: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("b.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {
            map,
            flushed: false,
        })
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        self.flushed = true;
        std::fs::write("b.db", contents)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flushed {
            let _ = self.flush();
        }
    }
}
