use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Search)-> Result<(),Box<dyn Error>>{
    //read file content to string and unwrap option
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query,&content)
    }else{
        search_case_insensitive(&config.query,&content)
    };

    for line in results{
        println!("{}", line);
    }
    Ok(())
        
}

//struct to hold a file path and the query on it to make a search
pub struct Search{
   pub query: String,
   pub file_path: String,
   pub case_sensitive: bool,
}

impl Search{
    //a function to create a config from the arguments list
    pub fn new(args: &[String]) -> Result<Search, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Search{query: args[1].clone(),file_path: args[2].clone(), case_sensitive})
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
Safe,Fast,Productive.
Pick three.
Duct tape";
        assert_eq!(
            vec!["Safe,Fast,Productive."],
            search_case_sensitive(query,contents))
    }
    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:
Safe,Fast,Productive.
Pick three.
Trust me";
        assert_eq!(
            vec!["Rust:","Trust me"],
            search_case_insensitive(query,contents))
    }
}
