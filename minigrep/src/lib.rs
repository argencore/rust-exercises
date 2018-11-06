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
    pub fn new(mut args: std::env::Args) -> Result<Search, &'static str>{
        //don't need process name
        args.next();

        let query = match args.next() {
            Some(args) => args,
            None => return Err("didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(args) => args,
            None => return Err("no file path specified"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Search{query,file_path, case_sensitive})
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

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
