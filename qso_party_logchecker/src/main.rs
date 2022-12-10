
mod arguments;
use crate::arguments::process_arguments;

fn main() {
    println!("Welcome to the Rust Qso Party LogChecker!");

    let config_file = process_arguments();    



    println!("All Done!");
}
