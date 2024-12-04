use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn main(enable: Option<bool>) {
    if !enable.unwrap_or(false) {
        return;
    }

    let input_path = "src/problem2/input_real.txt";
    let input = read_input(input_path);

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let result = reports.iter().filter(|report| is_safe(report)).count();

    //println!("Parsed input: {:?}", reports);
    println!("Result: {}", result);
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut i = 0;
    let mut j = 1;
    let mut increasing: Option<bool> = None;
    while j < report.len() {
        if report[i] == report[j] || (report[i] - report[j]).abs() > 3 {
            return false;
        } else if report[i] > report[j] {
            if increasing == Some(true) {
                return false;
            }
            increasing = Some(false);
        } else {
            if increasing == Some(false) {
                return false;
            }
            increasing = Some(true);
        }
        i += 1;
        j += 1;
    }
    true
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
