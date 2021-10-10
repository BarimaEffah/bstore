fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("you didn't provide a key");
    let value = arguments.next().expect("you didn't provide a value");
    println!("the key is {} and the value is {}", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("b.db", contents).unwrap();
}
