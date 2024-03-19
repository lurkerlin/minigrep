use minigrep::Config;
use minigrep::run;
use ansi_term::Colour::Blue;
fn main() {
    let config= Config::build(std::env::args()).unwrap_or_else(|e|{
        eprintln!("Problem parsing arguments: {}", e);
        std::process::exit(1);
    });
    println!("Searching for \"{}\" in File: {}",Blue.paint(&config.query),Blue.paint(&config.file_path));
    run(config).unwrap_or_else(|e|{
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    })
}

