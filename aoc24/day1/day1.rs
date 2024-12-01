use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let mut totalSum = 0;
    let mut similarityScore = 0;
    let mut intNumbersLeft = Vec::<i32>::new();
    let mut intNumbersRight = Vec::<i32>::new();
    let lines = read_to_string(filename).unwrap();
    for line in lines.lines() {
        let l = line.to_string();
        let numbers:Vec<&str> = l.split("   ").collect();
        intNumbersLeft.push(numbers[0].parse::<i32>().unwrap());
        intNumbersRight.push(numbers[1].parse::<i32>().unwrap());
    }

    intNumbersLeft.sort();
    intNumbersRight.sort();
    for i in 0..intNumbersLeft.len() {
        totalSum += (intNumbersRight[i] - intNumbersLeft[i]).abs()
    }

    for left in intNumbersLeft {
        let mut times = 0;
        for right in intNumbersRight.clone() {
            if left == right {
                times += 1;
            }
        }
        similarityScore += times * left;
    }
    println!("Part 1 = {}", totalSum);
    println!("Part 2 = {}", similarityScore);
}