use std::fs::read_to_string;

fn main() {
    let mut lines = Vec::new();
    let filename = "input.txt";
    let mut sum = 0;
    for line in read_to_string(filename).unwrap().lines() {
        sum += calculate(line.to_string());
        lines.push(line.to_string());
    }
    println!("{}", sum);
}

fn calculate(num: String) -> i64 {
    let number = num.parse::<i64>().unwrap();
    let mut result = number / 3 - 2;
    
    result
}
