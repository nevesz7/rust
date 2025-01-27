use std::env;
use std::process;
// use std::fs;

struct Config {
    search_string: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // if let Err(e) = grep::run(config) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // let contents = fs::read_to_string(config.file_path)
    // .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    println!("Search string: {}", config.search_string);
    println!("File path: {}", config.file_path);
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let search_string = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { search_string, file_path })
    }
}