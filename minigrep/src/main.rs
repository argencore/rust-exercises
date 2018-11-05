extern crate minigrep;

use std::process;
use std::env;
use minigrep::Search;

fn main() {
    //get iterator of command line arguments and then collect into a vector
    let args: Vec<String> = env::args().collect();
    let search = Search::new(&args).unwrap_or_else(|err|{
        eprintln!("problem parsing arguments:{}", err);
        process::exit(1)
    });
    if let Err(e) = minigrep::run(search){
        eprintln!("application encountered an error:{}",e);
        process::exit(1)
    }

}

