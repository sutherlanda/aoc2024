use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn main(enable: Option<bool>) {
    if !enable.unwrap_or(false) {
        return;
    }

    let input_path = "src/problem3/input_real.txt";
    let input = read_input(input_path);

    let pattern = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don\'t\(\))").unwrap();
    let exps: Vec<&str> = pattern.find_iter(&input).map(|m| m.as_str()).collect();
    let (result, _) = exps
        .iter()
        .fold((0, true), |(product, enabled), exp| match exp {
            &"do()" => (product, true),
            &"don't()" => (product, false),
            _ => {
                if !enabled {
                    return (product, enabled);
                }

                let mut mul_iter = exp.chars().skip(4);
                let a: u32 = mul_iter
                    .by_ref()
                    .take_while(|c| c != &',')
                    .collect::<String>()
                    .parse()
                    .unwrap();
                let b: u32 = mul_iter
                    .by_ref()
                    .take_while(|c| c != &')')
                    .collect::<String>()
                    .parse()
                    .unwrap();
                (product + a * b, enabled)
            }
        });

    println!("Problem 1: {}", input);
    println!("Problem 1: {:?}", exps);
    println!("Problem 1: {}", result);
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
