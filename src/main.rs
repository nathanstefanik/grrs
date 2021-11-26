use std::env;
use std::process;
use grrs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}:", config.pattern,config.path);
    
    if let Err(e) = grrs::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}




