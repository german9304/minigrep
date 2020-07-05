pub mod config;

use std::io;
use std::env;
use std::fs;
use std::error::Error;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_word() {
        let query = "test";
        let contents = "
        one 
        two
        three
test";

        assert_eq!(
            vec!["test"],
            search(&query.to_string(), &contents.to_string())
        )
    }
}

pub fn file_path(file_name: &String) -> Result<Option<String>, io::Error> {
    let dir = env::current_dir()?;
    let file_path_name = dir.join(file_name);
    if let Some(abs_path) = file_path_name.to_str() {
        return Ok(Some(String::from(abs_path)))
    }
    Ok(None)
}

pub fn search<'a>(query: &String, contents: &'a String) -> Vec<&'a str>{
    let mut results = Vec::new();

    for content in contents.lines() {
        if content.contains(query) {
            results.push(content);
        }
    }
    results
}


pub fn run(conf: &config::Config) -> Result<(), Box<dyn Error>> {
    let query = conf.query();
    let name = conf.filename();
    let dir_name = file_path(name)?
        .unwrap_or_else(|| panic!("error in directory"));
    let contents = fs::read_to_string(&dir_name)?;
    let results = search(query, &contents);

    for result in results {
        println!("{}", result);
    }
    Ok(())
}
