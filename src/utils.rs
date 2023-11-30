use std::fs;

pub fn lines_from_file(file_name: &str) -> Vec<String> {
    let s = fs::read_to_string(file_name).unwrap();
    s.lines().map(String::from).collect()
}


