use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let mut stones: Vec<&str> = input.split(" ").collect();
    let mut stringStones = Vec::<String>::new();
    let mut mapStones:HashMap<String, u64> = HashMap::new();
    let mut result = 0;
    for stone in stones {
        stringStones.push(stone.to_string());
        *mapStones.entry(stone.to_string()).or_insert(0) += 1;
    }
    for _ in 0..75 {
        let mut newStones = Vec::<String>::new();
        let mut newMapStones: HashMap<String, u64> = HashMap::new();
        for stone in mapStones.keys() {
            let mut ruleStones = applyRule(stone.clone());
            let mut val = mapStones.get(stone).unwrap();
            for rS in ruleStones {
                *newMapStones.entry(rS.clone()).or_insert(0) += val;
            }
        }
        mapStones = newMapStones.clone();
    }
    println!("{}", mapStones.values().sum::<u64>());
}

fn applyRule(s: String) -> Vec::<String> {
    let num = s.parse::<u64>().unwrap();
    let mut newStones = Vec::<String>::new();
    if num == 0 {
        newStones.push(String::from("1"));
    } else if s.len() % 2 == 0 {
        let mut start: String = s.chars().take(s.len()/2).collect();
        let mut end: String = s.chars().skip(s.len()/2).collect();
        let endNum = end.parse::<u64>().unwrap();
        let startNum = start.parse::<u64>().unwrap();
        end = endNum.to_string();
        start = startNum.to_string();
        newStones.push(start);
        newStones.push(end);
    } else {
        let num1 = num * 2024;
        newStones.push(num1.to_string());
    }
    return newStones;
}