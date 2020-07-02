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
        let text = "
        one 
        two
        three
        test";

        assert!(1 == 1);
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


pub fn run(conf: &config::Config) -> Result<(), Box<dyn Error>> {

    let query = conf.query();
    let name = conf.filename();

    match file_path(name) {
        Ok(dir) =>  {
            if let Some(dir_path) = dir {
            let contents = fs::read_to_string(&dir_path)?;
            println!("{}", contents);
            }
        }
        Err(err) => println!(" error here {}", err),
    }
    Ok(())
}
