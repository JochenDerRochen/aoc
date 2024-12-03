fn main() {
    let input = include_str!("input.txt");
    let mut result = 0;
    let mut result2 = 0;
    let mut take = true;
    for line in input.lines() {
        let mut corruptedData: String = line.clone().chars().collect();
        let v:Vec<_>  = corruptedData.match_indices("mul(").map(|(i,_)| i).collect();
        let dont:Vec<_> = corruptedData.match_indices("don't()").map(|(i, _)|i).collect();
        let dos:Vec<_> = corruptedData.match_indices("do()").map(|(i, _)|i).collect();
        for occurence in &v {
            let begin = occurence+3;
            let mut character = '(';
            let mut offset = 0;
            let mut mult = String::from("");
            while character != ')' {
                if let Some(ok) = corruptedData.chars().nth(begin+offset) {
                    character = ok;
                }
                else {
                    break;
                }
                mult.push(character);
                offset += 1;
            }
            result += calculateMult(&mult);
        }
        for i in 0..line.len() {
            if dont.contains(&i) {
                take = false;
            }
            if dos.contains(&i) {
                take = true;
            }
            if v.contains(&i) && take{
                let begin = i+3;
                let mut character = '(';
                let mut offset = 0;
                let mut mult = String::from("");
                while character != ')' {
                    if let Some(ok) = corruptedData.chars().nth(begin+offset) {
                        character = ok;
                    }
                    else {
                        break;
                    }
                    mult.push(character);
                    offset += 1;
                }
                result2 += calculateMult(&mult);
            }
        }
    }
    println!("Part 1 = {}", result);
    println!("Part 2 = {}", result2);
}

fn calculateMult(s: &String) -> u32 {
    if s.chars().any(|c| matches!(c, 'a'..='z')) {
        return 0
    }
    if !s.chars().any(|c| matches!(c, ',')) {
        return 0
    }
    let split = s.find(",").unwrap();
    let firstNumber:String = s.chars().skip(1).take(split-1).collect();
    let mut number2 = 0;
    let mut number1 = 0;
    
    match firstNumber.parse::<u32>() {
        Ok(my_int) => number1 = my_int,
        Err(err) => return 0,
    }
    let secondNumber:String = s.chars().skip(split+1).take(s.len()-split-2).collect();

    match secondNumber.parse::<u32>() {
        Ok(my_int) => number2 = my_int,
        Err(err) => return 0,
    }
    return number1* number2;
}