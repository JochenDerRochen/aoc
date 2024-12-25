fn main() {
    let input = include_str!("input.txt");
    let mut numbers = Vec::<i64>::new();
    let mut result = 0;
    for line in input.lines() {
        numbers.push(line.parse::<i64>().unwrap());
    }
    for number in numbers {
        let mut secNum = number;
        for i in 0..2000 {
            secNum = calculateSecretNumbers(secNum);
        }
        result += secNum
    }
    println!("{:?}", result);
}

fn calculateSecretNumbers(oldNum: i64) -> i64 {
    let mut result: i64 = oldNum;
    result *= 64;
    result = oldNum ^ result;
    result = prune(result);
    let mut div = result / 32;
    result = div ^ result;
    result = prune(result);
    let mut mult = result * 2048;
    result = mult ^ result;
    result = prune(result);
    return result;
}

fn prune(num: i64) -> i64 {
    if num == 100000000 {
        return 16113920;
    } else {
        return num % 16777216;
    }
}