use std::{error::Error, fs};

/// Once we have two valid command-line arguments, runs the rest of the program.
pub fn run(query: &str, path: &str, ignore_case: bool) -> Result<(), Box<dyn Error>> {
    let contents = &fs::read_to_string(path)?;
    let results = if !ignore_case {
        search(query, contents)
    } else {
        search_case_insensitive(query, contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic(expected = "No such file or directory")]
    #[test]
    fn fail_on_file_not_found() {
        run("unnecessary", "NO_FILE.txt", true).unwrap();
    }

    #[should_panic(expected = "did not contain valid UTF-8")]
    #[test]
    fn fail_on_file_cannot_open_or_read() {
        run("unnecessary", "../../include/google-trends-2022.png", true).unwrap();
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = vec![
            "Rust:",
            "safe, fast, productive.",
            "Pick three.",
            "Duct tape.",
        ]
        .join("\n");
        assert_eq!(vec!["safe, fast, productive."], search(query, &contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = vec![
            "Rust:",
            "safe, fast, productive.",
            "Pick three.",
            "Duct tape.",
        ]
        .join("\n");
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query, &contents)
        );
    }
}
