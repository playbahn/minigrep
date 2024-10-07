use std::{fs, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {        
    pub fn build(
        mut env_args_iter: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        env_args_iter.next();

        let query: String = match env_args_iter.next() {
            Some(query_string) => query_string,
            None => return Err("Didn't get a query string"),
        };

        let file_path: String = match env_args_iter.next() {
            Some(file_path) => file_path,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case: bool = match env::var("MINIGREP_DEFY_CASE")
                .unwrap_or(String::from(|ignore_flag: Option<String>| -> &'static str {
                    // match ignore_flag {
                    //     Some(S) => todo!(),
                    //     None => todo!(),
                    // }

                    return if 
                    String::from("defy-case=1") == 
                    ignore_flag.unwrap_or(String::from("")) {
                        "1"
                    } else {
                        "0"
                    }

                    // return if 4 == env_args.len() && "defy-case=1" == env_args[3].as_str() {
                    //     "1"
                    // } else {
                    //     "0"
                    // }            
                } (env_args_iter.next()))).as_str() {
            "1"  => { true },
            &_   => { false },
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let search_results: Vec<&'_ str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in search_results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<&'a str>>()
    // let mut search_results: Vec<&'a str> = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(&query) {
    //         search_results.push(line);
    //     }
    // }

    // search_results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query: String = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect::<Vec<&'a str>>()

    // let mut search_results: Vec<&'a str> = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         search_results.push(line);
    //     }
    // }

    // search_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let test_query: String = String::from("duct");
        let test_contents: String = String::from("\
Rust:
safe, fast, productive.
Pick three.
Duct tape.");

        assert_eq!(vec!["safe, fast, productive."], search(&test_query, &test_contents));
    }

    #[test]
    fn case_insensitive() {
        let test_query: String = String::from("rUsT");
        let test_contents: String = String::from("\
Rust:
safe, fast, productive.
Pick three.
Trust me.");

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&test_query, &test_contents)
        );
    }
}