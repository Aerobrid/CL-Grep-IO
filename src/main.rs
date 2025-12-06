use std::env;

fn main() {
    // explicit type annotation here since .collect() can return many types of collections
    // and Rust compiler can't infer which one you want
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}")
}
