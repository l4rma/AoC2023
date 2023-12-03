use crate::utils::lines_from_file;
use std::collections::HashMap;

// only 12 red cubes, 13 green cubes, and 14 blue cubes
pub fn main() {
        let lines = lines_from_file("inputs/day2.txt");
        let mut result = 0;

        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let mut i = 1;

        for line in lines {
            // println!("{}", line);
        let mut color_map: HashMap<String, i32> = HashMap::new();
            let reveales: Vec<&str> = line[line.find(":").unwrap()+1..].split(";").collect();
            for revealed in reveales {
                let colors: Vec<&str> = revealed.split(",").collect();
                for color in colors {
                    // println!("{}", color.trim());
                    let color_name: String = color.trim()[color.trim().find(" ").unwrap()+1..].to_string();
                    let number: i32 = color.trim()[..color.trim().find(" ").unwrap()].to_string().parse::<i32>().unwrap();
                    if color_map.is_empty() {
                        color_map.insert(color_name, number);
                    } else if color_map.get(&color_name).is_none() || color_map.get(&color_name).expect("msg") < &number {
                        color_map.insert(color_name, number);
                    }
                }
            }
            // check if over max and sum
            let red = color_map.get("red").expect("no reds");
            let green = color_map.get("green").expect("no greens");
            let blue = color_map.get("blue").expect("no blues");
            
            if red <= &max_red && blue <= &max_blue && green <= &max_green {
                // sum up ids
                result += i;
            }
            println!("Game id: {:?}", i);
            i+=1;
            println!("Sub sum: {:?}", result);

        }
    println!("Total: {:?}", result);
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn part1_example() {
        let example1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        let lines: Vec<String> = example1.lines().map(String::from).collect();
        let expected = 8;
        let mut result = 0;

        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let mut i = 1;

        for line in lines {
            // println!("{}", line);
        let mut color_map: HashMap<String, i32> = HashMap::new();
            let reveales: Vec<&str> = line[line.find(":").unwrap()+1..].split(";").collect();
            for revealed in reveales {
                let colors: Vec<&str> = revealed.split(",").collect();
                for color in colors {
                    // println!("{}", color.trim());
                    let color_name: String = color.trim()[color.trim().find(" ").unwrap()+1..].to_string();
                    let number: i32 = color.trim()[..color.trim().find(" ").unwrap()].to_string().parse::<i32>().unwrap();
                    if color_map.is_empty() {
                        color_map.insert(color_name, number);
                    } else if color_map.get(&color_name).is_none() || color_map.get(&color_name).expect("msg") < &number {
                        color_map.insert(color_name, number);
                    }
                }
            }
            // check if over max and sum
            let red = color_map.get("red").expect("no reds");
            let green = color_map.get("green").expect("no greens");
            let blue = color_map.get("blue").expect("no blues");
            
            if red < &max_red && blue < &max_blue && green < &max_green {
                // sum up ids
                result += i;
            }
            i+=1;
            println!("{:?}", i);
            println!("{:?}", result);

        }

        assert_eq!(expected, result);
    }

    #[test]
    fn part2_example() {
        let example2 = "";

        let lines: Vec<String> = example2.lines().map(String::from).collect();
        let expected = 0;

        for line in lines {
            todo!("{}", line);
        }

        let result = 0;
        assert_eq!(expected, result);
    }
}


