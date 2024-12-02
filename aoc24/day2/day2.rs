fn main() {
    let lines = include_str!("input.txt");
    let mut counter = 0;
    let mut counter2 = 0;
    for line in lines.lines() {
        let mut numbers = Vec::<i32>::new();
        let mut levels:Vec<&str> = line.split_whitespace().collect();
        for level in levels {
            numbers.push(level.parse().unwrap());
        }
        if checkIfSafe(&numbers) {
            counter += 1;
            counter2 += 1;
        } else {
            if checkIfSafe2(numbers) {
                counter2 += 1;
            }
        }
    }
    println!("{}", counter);
    println!("{}", counter2);
}

fn checkIfSafe(l:&Vec<i32>) -> bool {
    let mut distances = Vec::<i32>::new();
    for i in 0..l.len()-1 {
        distances.push(l[i] - l[i+1]);
    }
    let mut currentNZDistance = 0;
    for index in 0..distances.len() {
        if distances[index] < -3 || distances[index] > 3 || distances[index] == 0{
            return false;
        } else {
            if currentNZDistance > 0 && distances[index] < 0 || currentNZDistance < 0 && distances[index] > 0 {
                return false
            }
            if distances[index] != 0 {
                currentNZDistance = distances[index];
            }
        }
    }
    return true
}

fn checkIfSafe2(l:Vec<i32>) -> bool {
    for i in 0..l.len() {
        let mut distances = Vec::<i32>::new();
        let mut lCop = l.clone();
        lCop.remove(i);
        if checkIfSafe(&lCop) {
            return true
        }
    }
    return false;
}