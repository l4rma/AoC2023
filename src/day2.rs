//use crate::utils::lines_from_file;

pub fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let example1 = "asdf
ashhhasdf
ashhhasdfas
asdfjj
lkjskjflll";

        let lines: Vec<String> = example1.lines().map(String::from).collect();
        let expected = 0;

        for line in lines {
            dbg!(line);
        }

        let result = 0;
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


