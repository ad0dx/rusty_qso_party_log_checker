
mod arguments;
mod file_utils;
mod utils;

use crate::arguments::process_arguments;
//use crate::utils::exit_process;

use std::env;

fn main() {
    println!("Welcome to the Rust Qso Party LogChecker by ad0dx!");

    let args: Vec<String> = env::args().collect();
    let config_file_result = process_arguments(&args);    

    let _config_file = match config_file_result {
        Ok(filename) => filename,
        Err(e) => {utils::exit_process(e); "".to_string()}
    };


    println!("All Done!");
}
