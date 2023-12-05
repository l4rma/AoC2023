use crate::utils::lines_from_file;

pub fn main() {
    let lines = lines_from_file("inputs/day4.txt");
    let result_part1 = get_result_part1(lines.clone());
    let result_part2 = get_result_par2(lines.clone());
    println!("(Part1) total: {:?}", result_part1);
    println!("(Part2) total: {:?}", result_part2);
}

pub fn get_result_part1(lines: Vec<String>) -> i32 {
    let mut result = 0;

    for line in lines {
        let (winning_part, your_part) = line[line.find(": ").unwrap()+2..].split_once(" | ").unwrap();
        let winning_numbers: Vec<i64> = winning_part.split_whitespace().map(|n| n.parse().expect("Failed to parse")).collect();
        let mut points = 0;
        for w in winning_numbers {
            let your_numbers: Vec<i64> = your_part.split_whitespace().map(|n| n.parse().expect("Failed to parse")).collect();
            for n in your_numbers {
                if n == w {
                    if points == 0 {
                        points += 1;
                    } else {
                        points += points;
                    }
                }
            }
        }
        result += points;
    }
    return result;
}

fn get_result_par2(lines: Vec<String>) -> i32 {
    let result = 0;

    for _line in lines {
    }
    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_4_part1_example() {
        let example1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let lines: Vec<String> = example1.lines().map(String::from).collect();
        let expected = 13;
        let mut result = 0;
        for line in lines {
            let (winning_part, your_part) = line[line.find(": ").unwrap()+2..].split_once(" | ").unwrap();
            let winning_numbers: Vec<i64> = winning_part.split_whitespace().map(|n| n.parse().expect("Failed to parse")).collect();
            let mut points = 0;
            for w in winning_numbers {
                let your_numbers: Vec<i64> = your_part.split_whitespace().map(|n| n.parse().expect("Failed to parse")).collect();
                for n in your_numbers {
                    if n == w {
                        if points == 0 {
                            points += 1;
                        } else {
                            points += points;
                        }
                    }
                }
            }
            result += points;
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


