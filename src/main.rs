use std::env;
use std::io;


fn main() {

    let file_name: Vec<String> = env::args().collect();

    let file_path_result = myminigrep::file_path(&file_name[1]);

    match file_path_result {
        Ok(result) =>  {
            if let Some(path) = result {
                println!("current path: {}", path)
            }
        },
        Err(err) => println!("{}", err),
    }
}
