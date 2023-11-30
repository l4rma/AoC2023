use crate::utils::lines_from_file;

pub fn main() {
    let lines = lines_from_file("inputs/day0.txt");
    for line in lines {
        println!("{}", line);
    }

}
