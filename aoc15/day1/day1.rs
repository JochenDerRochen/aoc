use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let mut floor = 0;
    for line in read_to_string(filename).unwrap().lines() {
        for (idx, c) in line.chars().enumerate() {
            if(c == '(') {
                floor += 1;
            }
            else if(c == ')') {
                floor -= 1;
            }
            if(floor == -1) {
                println!("{}", idx);
                break;
            }
        }
    }
    println!("{}", floor);
}
