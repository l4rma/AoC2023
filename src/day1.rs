use crate::utils::lines_from_file;

pub fn main() {
    let mut total_sum = 0;
    let patterns = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let lines = lines_from_file("inputs/day1.txt");
    for line in lines {
        // PART 1
        // let sum_in_line = get_sum_from_line(&line);
        // total_sum += sum_in_line;

        //PART 2
        let first = as_number(find_first(&line, &patterns).unwrap_or("default"));
        let last = as_number(find_last(&line, &patterns).unwrap_or("default"));
        let sum = first.to_owned() + last;
        // print!("{:?} + ", first);
        // print!("{:?} = ", last);
        // println!("{:?}", sum.parse::<i32>().unwrap_or(0));
        total_sum += sum.parse::<i32>().unwrap();
    }
    println!("Total sum is: {}", total_sum);
}

pub fn as_number(string: &str) -> &str {
    match string {
        "one" | "1" => return "1",
        "two" | "2" => return "2",
        "three" | "3" => return "3",
        "four" | "4" => return "4",
        "five" | "5" => return "5",
        "six" | "6" => return "6",
        "seven" | "7" => return "7",
        "eight" | "8" => return "8",
        "nine" | "9" => return "9",
        _ => return "0",
    }
} 

pub fn find_last<'a>(input: &'a str, patterns: &'a [&'a str]) -> Option<&'a str> {
    let mut last_occurrence: Option<&'a str> = None;
    for pattern in patterns {
        if let Some(index) = input.rfind(pattern) {
            if let Some(existing_index) = last_occurrence.map(|existing| input.rfind(existing).unwrap_or(input.len())) {
                if index > existing_index {
                    last_occurrence = Some(&input[index..(index + pattern.len())]);
                }
            } else {
                last_occurrence = Some(&input[index..(index + pattern.len())]);
            }
        }
    }
    last_occurrence
}

pub fn find_first<'a>(input: &'a str, patterns: &'a [&'a str]) -> Option<&'a str> {
    let mut first_occurrence: Option<&'a str> = None;
    for pattern in patterns {
        if let Some(index) = input.find(pattern) {
            if let Some(existing_index) = first_occurrence.map(|existing| input.find(existing).unwrap_or(input.len())) {
                if index < existing_index {
                    first_occurrence = Some(&input[index..(index + pattern.len())]);
                }
            } else {
                first_occurrence = Some(&input[index..(index + pattern.len())]);
            }
        }
    }
    first_occurrence
}

#[allow(dead_code)]
pub fn get_sum_from_line(line: &str) -> i32{
    let mut sum: String = String::new();
    let radix: u32 = 10;
    let first: i32;
    let last: i32;
    // First number
    for c in line.chars() {
        if c.is_digit(radix) {
            first = c.to_digit(radix).unwrap_or(0) as i32;
            sum.push(c);
            print!("{} + ", first);
            break;
        }
    }
    // Last number
    for c in line.chars().rev() {
        if c.is_digit(radix) {
            last = c.to_digit(radix).unwrap_or(0) as i32;
            sum.push(c);
            print!("{}", last);
            break;
        }
    }
    println!(" = {:?}", sum.parse::<i32>().unwrap_or(0));
    sum.parse::<i32>().unwrap_or(0)
}
