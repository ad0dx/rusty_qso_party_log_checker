
use std::path::Path;

// Return true if the given file exists
pub fn file_exists(filename: &String) -> bool {
    Path::new(filename).exists()
}
