use std::env;
use std::fs;

fn main() {
    // explicit type annotation here since .collect() can return many types of collections
    // and Rust compiler can't infer (implicit) which one you want
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
