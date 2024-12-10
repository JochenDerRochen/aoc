fn main() {
    let mut result = 0;
    let input = include_str!("input.txt");
    for line in input.lines() {
        result += checkIfLineIsDoable(line);
    }
    println!("{}", result);
}

fn concat(a: i64, b: i64) -> i64 {
    a as i64 * 10i64.pow(b.ilog10() + 1) + b as i64
}

fn checkIfLineIsDoable(line:&str) -> i64 {
    let mut results = Vec::<Vec<i64>>::new();
    let numbers: Vec<_> = line.split(" ").collect();
    let mut s = String::from("");
    let mut resultNum = 0;
    let mut variables = Vec::<i64>::new();
    for i in 0..numbers.len() {
        if numbers[i].contains(":") {
            s = numbers[i].chars().take(numbers[i].len()-1).collect::<Vec<_>>().iter().collect();
            resultNum = s.parse::<i64>().unwrap();
        } else {
            variables.push(numbers[i].parse::<i64>().unwrap());
        }
    }
    for j in 0..variables.len() {
        let mut possibleResults = Vec::<i64>::new();
        if j > 1 {
            for result in results[j-2].clone() {
                possibleResults.push(variables[j]+result);
                possibleResults.push(variables[j]*result);
                possibleResults.push(concat(result, variables[j]));
            }
            results.push(possibleResults);
        } else if j == 0 {
            //if variables[j] * variables[j+1] == resultNum || variables[j] + variables[j+1] == resultNum && variables.len() == 2{
             //   return resultNum;
            //}
            possibleResults.push(variables[j] * variables[j+1]);
            possibleResults.push(variables[j] + variables[j+1]);
            possibleResults.push(concat(variables[j], variables[j+1]));
            results.push(possibleResults);
        }
    }
    //println!("{:?}", results[results.len()-1]);
    if results[results.len()-1].contains(&resultNum) {
        return resultNum;
    }

    return 0;
}