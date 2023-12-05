use crate::utils::lines_from_file;
// use std::collections::HashMap;

pub fn main() {
    let lines = lines_from_file("inputs/day3.txt");
    let result_part1 = get_result_part1(lines.clone());
    let result_part2 = get_result_par2(lines.clone());

    println!("(Part1) total: {:?}", result_part1);
    println!("(Part2) total: {:?}", result_part2);
}

pub fn get_result_part1(lines: Vec<String>) -> i32 {
    let mut result = 0;

        let mut i = 0;
        let mut parts: Vec<String> = Vec::new();

        loop {
            if i == lines.len() {
                break;
            }
            let mut prev_line = Vec::new();
            if i > 1 {
                prev_line = lines[i-1].chars().collect();
            }
            let current_line: Vec<char> = lines[i].chars().collect();
            let mut next_line = Vec::new();
            if i < lines.len() - 1 {
                next_line = lines[i+1].chars().collect();
            }

            let mut j = 0;
            while j < current_line.len() {
                let mut number = String::new();
                let mut is_part: bool = false;
                if current_line[j].is_digit(10) {
                    number.push(current_line[j]);
                }
                while j < current_line.len() - 1 && current_line[j+1].is_digit(10) {
                    if prev_line.len() != 0 {
                        if prev_line[j] != '.' && !prev_line[j].is_digit(10) {
                            is_part = true;
                        }
                        if j+1 <  prev_line.len() && prev_line[j+1] != '.' && !prev_line[j+1].is_digit(10) {
                            is_part = true;
                        }
                        if j+2 <  prev_line.len() && prev_line[j+2] != '.' && !prev_line[j+2].is_digit(10) {
                            is_part = true;
                        }
                    }

                    if j > 0 && current_line[j] != '.' && !current_line[j].is_digit(10) {
                        is_part = true;
                    }
                    if j+2 < current_line.len() && current_line[j+2] != '.' && !current_line[j+2].is_digit(10) {
                        is_part = true;
                    }

                    if next_line.len() != 0 {
                        if next_line[j] != '.' && !next_line[j].is_digit(10) {
                            is_part = true;
                        }
                        if j+1 <  next_line.len() && next_line[j+1] != '.' && !next_line[j+1].is_digit(10) {
                            is_part = true;
                        }
                        if j+2 <  next_line.len() && next_line[j+2] != '.' && !next_line[j+2].is_digit(10) {
                            is_part = true;
                        }
                    }
                    number.push(current_line[j+1]);
                    j += 1;
                }
                if is_part {
                    parts.push(number);
                }
                j += 1;
            }
            i += 1;
        }
        for num in parts {
            match num.parse::<i32>() {
                Ok(n) => result += n,
                Err(err) => println!("Error: {:?}", err),
            };

        }
    return result;
}

fn get_result_par2(lines: Vec<String>) -> i32 {
    let result = 0;

    for line in lines {
        println!("{}", line);
    }
    return result;
}


#[cfg(test)]
mod tests {
    #[test]
    fn day_3_part1_example() {
        let example1 = "467..114..
...*......
..35..633.
.........#
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let lines: Vec<String> = example1.lines().map(String::from).collect();
        let expected = 4361;
        let mut result = 0;
        let mut i = 0;
        let mut parts: Vec<String> = Vec::new();

        loop {
            if i == lines.len() {
                break;
            }
            let mut prev_line = Vec::new();
            if i > 1 {
                prev_line = lines[i-1].chars().collect();
            }
            let current_line: Vec<char> = lines[i].chars().collect();
            let mut next_line = Vec::new();
            if i < lines.len() - 1 {
                next_line = lines[i+1].chars().collect();
            }

            let mut j = 0;
            while j < current_line.len() {
                let mut number = String::new();
                let mut is_part: bool = false;
                if current_line[j].is_digit(10) {
                    number.push(current_line[j]);
                }
                while j < current_line.len() - 1 && current_line[j+1].is_digit(10) {
                    if prev_line.len() != 0 {
                        if prev_line[j] != '.' && !prev_line[j].is_digit(10) {
                            is_part = true;
                        }
                        if j+1 <  prev_line.len() && prev_line[j+1] != '.' && !prev_line[j+1].is_digit(10) {
                            is_part = true;
                        }
                        if j+2 <  prev_line.len() && prev_line[j+2] != '.' && !prev_line[j+2].is_digit(10) {
                            is_part = true;
                        }
                    }

                    if j > 0 && current_line[j] != '.' && !current_line[j].is_digit(10) {
                        is_part = true;
                    }
                    if j+2 < current_line.len() && current_line[j+2] != '.' && !current_line[j+2].is_digit(10) {
                        is_part = true;
                    }

                    if next_line.len() != 0 {
                        if next_line[j] != '.' && !next_line[j].is_digit(10) {
                            is_part = true;
                        }
                        if j+1 <  next_line.len() && next_line[j+1] != '.' && !next_line[j+1].is_digit(10) {
                            is_part = true;
                        }
                        if j+2 <  next_line.len() && next_line[j+2] != '.' && !next_line[j+2].is_digit(10) {
                            is_part = true;
                        }
                    }
                    number.push(current_line[j+1]);
                    j += 1;
                }
                if is_part {
                    print!("{} ",number);
                    parts.push(number);
                }
                j += 1;
            }
            println!("");
            i += 1;
        }
        for num in parts {
            match num.parse::<i32>() {
                Ok(n) => result += n,
                Err(err) => println!("Error: {:?}", err),
            };

        }
        assert_eq!(expected, result);
    }

    #[test]
    fn part2_example() {
        let example1 = "";

        let lines: Vec<String> = example1.lines().map(String::from).collect();
        let expected = 0;
        let result = 0;

        for line in lines {
            println!("{}", line);
        }

        assert_eq!(expected, result);
    }
}


