use std::fs;
use itertools::Itertools;

pub fn lines_from_file(file_name: &str) -> Vec<String> {
    let s = fs::read_to_string(file_name).unwrap();
    s.lines().map(|l| l.split(' ').collect()).collect_vec()
}
