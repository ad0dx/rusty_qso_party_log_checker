
use std::path::Path;

pub fn file_exists(filename: &String) -> bool {
    Path::new(filename).exists()
}
