use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let query =  &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In the file {file_path}");
}
