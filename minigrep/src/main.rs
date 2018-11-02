use std::env;
use std::fs;

fn main() {
    //get iterator of command line arguments and then collect into a vector
    let args: Vec<String> = env::args().collect();

    // the values are set as immutable borrows, because there is no reason to copy or change the values
    let config = parse_config(&args);
  
    println!("in file {}", config.file_path);
    //read file content to string and unwrap option
    let content = fs::read_to_string(config.file_path).expect("could not read from file");

    println!("with text:\n{}", content);
}

//struct to hold a file path and the query on it
//TODO: come up with a better name
struct Config{
    query: String,
    file_path: String,
}

//a function to create a config from the arguments list
fn parse_config(args: &[String])-> Config{
    //TODO: decide if I can pass ownership instead of clone
    Config {query: args[1].clone(),file_path: args[2].clone()}
}
