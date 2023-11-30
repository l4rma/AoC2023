use crate::utils::lines_from_file;

pub fn main() {
    let lines = lines_from_file("inputs/day0.txt");
    let mut calories_list: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for line in lines {
        if line == "" {
            calories_list.push(sum);
            sum = 0;
            //println!("such empty");
        } else {
            let _calories: i32 = line.parse().unwrap();
            sum += _calories;
            //println!("{:?}", _calories);
        }
    }
    // for num in &calories_list {
    //     println!("{:?}", num)
    // }
    calories_list.sort();
    println!("Highest number: {:?}", calories_list.get(calories_list.len()-1).unwrap());

}
