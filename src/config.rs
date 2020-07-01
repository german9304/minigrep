#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let query = String::from("test");
        let filename = String::from("water");

        let config = Config::new(query, filename);

        assert_eq!(config.query().as_str(), "test");
        assert_eq!(config.filename().as_str(), "water");
    }
}

pub struct Config {
    query: String,
    filename: String,
}


impl Config {

    fn new(query: String, filename: String) -> Config {

        Config {
            query, 
            filename
        }
    }

    fn query(&self) -> &String {
        &self.query
    }

    fn filename(&self) -> &String {
        &self.filename
    }
}
