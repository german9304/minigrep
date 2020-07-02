use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let query = String::from("test");
        let filename = String::from("water");
    }
}

pub struct Config {
    query: String,
    filename: String,
}


impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, String> {

        if args.len() < 2 {
            return Err(String::from("please provide more arguments"));
        }
        // iterator
        args.next();

        let query = if let Some(arg) = args.next() {
            arg
        } else {
              return Err(String::from("Please provide query"))
        };

        let filename = if let Some(arg) = args.next() {
            arg
        } else {
              return Err(String::from("Please provide query"))
        };
        
        Ok(Config {
            query, 
            filename
        })
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }
}
