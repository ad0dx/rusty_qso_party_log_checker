
mod arguments;
mod file_utils;
use crate::arguments::process_arguments;

fn main() {
    println!("Welcome to the Rust Qso Party LogChecker by ad0dx!");

    let config_file = process_arguments();    



    println!("All Done!");
}
