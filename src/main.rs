use std::process;
use std::env;
use minigrep::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensetive() {
        let query = "duct";
        let contents = "\
Rust:
safe, face, productive
Pick three.";

        assert_eq!(vec!["safe, face, productive"], search(query, contents));
    }

    #[test]
    fn case_insensetive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, face, productive
Pick three.";

        assert_eq!(vec!["Rust:"], search_case_insensetive(query, contents));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing parameters: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    };

}

