#[cfg(test)]
mod tests {
    use ansi_term::{Colour, Style};
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            super::search(query, contents)
        );
    }

    #[test]
    fn test_case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            super::search_case_insensitive(query, contents)
        );
    }
    #[test]
    pub fn test_ansi_term() {
        println!(
            "How about some {} and {} ro add some {}?",
            Style::new().bold().paint("bold"),
            Style::new().underline().paint("underline"),
            Colour::Red.paint("Red")
        );
        println!(
            "esay to combine colour and other style {} {}",
            Colour::Red.bold().paint("like this"),
            Colour::Blue.underline().paint("or this")
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

use ansi_term::Colour;
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let _ = ansi_term::enable_ansi_support();
    let contents = std::fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    results.iter().for_each(|line| {
        println!(
            "{}",
            line.replace(&config.query, &Colour::Red.underline().paint(&config.query).to_string())
        );
    });
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = args.next().ok_or("Query string not found")?;
        let file_path = args.next().ok_or("File path not found")?;
        let ignore_case = std::env::var("IGNORE_CASE").is_ok() || args.any(|arg| arg == "-i" || arg == "--ignore-case");
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
