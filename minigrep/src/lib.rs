use std::fs;
use std::error::Error;

pub fn run(search: Search)-> Result<(),Box<dyn Error>>{
    println!("in file {}", search.file_path);
    //read file content to string and unwrap option
    let content = fs::read_to_string(search.file_path)?;

    println!("with text:\n{}", content);
    Ok(())
        
}

//struct to hold a file path and the query on it to make a search
pub struct Search{
   pub query: String,
   pub file_path: String,
}

impl Search{
    //a function to create a config from the arguments list
    pub fn new(args: &[String]) -> Result<Search, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Search{query: args[1].clone(),file_path: args[2].clone()})
    }
}
