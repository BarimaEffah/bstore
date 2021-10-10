use std::collections::HashMap;
fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("you didn't provide a key");
    let value = arguments.next().expect("you didn't provide a value");
    println!("the key is {} and the value is {}", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("b.db", contents).unwrap();
    let database = Database::new().expect("Database crashed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        //read the db file
        //parse string from file
        //populate our map

        // let contents = match std::fs::read_to_string("b.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };

        let contents = std::fs::read_to_string("b.db")?;

        Ok(Database {
            map: HashMap::new(),
        })
    }
}
