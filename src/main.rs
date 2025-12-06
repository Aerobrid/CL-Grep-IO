use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;

fn main() {
    // explicit type annotation here since .collect() can return many types of collections
    // and Rust compiler can't infer (implicit) which one you want
    let args: Vec<String> = env::args().collect();

    // || {...} syntax -> closure -> anonymous functions in Rust
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // in case of an error return said error back to caller (main) with ?
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        print!("{line}\n");
    }  

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // in case of Err Rust compiler needs to know lifetime of str slice (static for entire duration of program)
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // we create deep copy with .clone() to avoid managing lifetime references/issues (negligible tradeoff)
        // if you want overkill you could annotate lifetimes 
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
