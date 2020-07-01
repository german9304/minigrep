mod config;

use std::io;
use std::env;



pub fn file_path(file_name: &String) -> Result<Option<String>, io::Error> {

    let dir = env::current_exe()?;

    let file_path_name = dir.join(file_name);

    if let Some(abs_path) = file_path_name.to_str() {
        return Ok(Some(String::from(abs_path)))
    }

    Ok(None)
}
