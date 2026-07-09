use std::env;
use std::fs;
use std::process;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(&config.file_path).expect("Something went wrong reading the file");

    let data = search(&config.query, &contents);
    println!("{:?}", data);
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(query: String, file_path: String) -> Self {
        Self { query, file_path }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config::new(args[1].clone(), args[2].clone()))
    }
}
