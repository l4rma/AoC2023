// use crate::utils::lines_from_file;
// use std::collections::HashMap;

pub fn main() {
    let lines = lines_from_file("inputs/day2.txt");
    let result_par1 = get_result_part1(lines.clone());
    let result_part2 = get_result_par2(lines.clone());
    
    println!("(Part1) total: {:?}", result_part1);
    println!("(Part2) total: {:?}", result_part2);
}

pub fn get_result_part1(lines: Vec<String>) -> i32 {
    let mut result = 0;

    for line in lines {
        println!("{}", line);
    }
    return result;
}

fn get_result_par2(lines: Vec<String>) -> i32 {
    let mut result = 0;

    for line in lines {
        println!("{}", line);
    }
    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let example1 = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let lines: Vec<String> = example1.lines().map(String::from).collect();
        let expected = 0;
        let mut result = 0;
        let mut i = 1;

        while(true) {
        }

        assert_eq!(expected, result);
    }

    #[test]
    fn part2_example() {
        let example1 = "";

        let lines: Vec<String> = example1.lines().map(String::from).collect();
        let expected = 0;
        let mut result = 0;

        for line in lines {
            todo();
        }

        assert_eq!(expected, result);
    }
}


