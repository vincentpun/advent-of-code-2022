use std::fs;
use std::path::Path;
use std::io::Result;

pub fn read_contents<P: AsRef<Path>>(path: P) -> Result<String> {
    fs::read_to_string(path)
}
