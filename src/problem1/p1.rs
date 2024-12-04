use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn main(enable: Option<bool>) {
    if !enable.unwrap_or(false) {
        return;
    }

    let input_path = "src/problem1/input_real.txt";
    let input = read_input(input_path);
    println!("Input:\n{}", input);

    let (mut left_nums, mut right_nums): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace();
            (
                nums.next().unwrap().parse::<i32>().unwrap(),
                nums.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    left_nums.sort();
    right_nums.sort();

    let differences: Vec<i32> = left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(l, r)| (l - r).abs())
        .collect();

    let sum = differences.iter().sum::<i32>();

    println!("Parsed input: {:?} {:?}", left_nums, right_nums);
    println!("Differences: {:?}", differences);
    println!("Sum of differences: {}", sum);
}

fn read_input(input_path: &str) -> String {
    let path = Path::new(input_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("unable to read file contents of {}: {}", display, why),
        Ok(_) => s,
    }
}