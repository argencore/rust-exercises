use std::env;
use std::fs;

fn main() {
    //get iterator of command line arguments and then collect into a vector
    let args: Vec<String> = env::args().collect();

    // the values are set as immutable borrows, because there is no reason to copy or change the values
    let query = &args[1];
    let file_path = &args[2];

    println!("in file {}", file_path);
    let content = fs::read_to_string(file_path).expect("could not read from file");

    println!("with text:\n{}", content);
}
