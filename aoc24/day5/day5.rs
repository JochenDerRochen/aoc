use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let mut result = 0;
    let mut result2 = 0;
    let mut rules = vec![vec![String::from("");0]; 0];
    for line in input.lines() {
        if line.contains("|") {
            let pages:Vec<&str> = line.split("|").collect();
            let firstPage = pages[0].to_string();
            let secondPage = pages[1].to_string();
            let rule = vec![firstPage, secondPage];
            rules.push(rule);
        } else if line.contains(",") {
            let mut ordering:Vec<String> = line.split(",").map(|i| i.to_string()).collect();
            let mut foundPages:Vec<String> = Vec::new();
            foundPages.push(ordering[0].clone());
            let mut wrong = false;
            for r in &rules {
                if ordering.contains(&r[0]) && ordering.contains(&r[1]) {
                    let index1 = ordering.iter().position(|i| *i == r[0]).unwrap();
                    let index2 = ordering.iter().position(|i| *i == r[1]).unwrap();
                    if index2 < index1 {
                        wrong = true;
                        break;
                    }
                }
            }
            let mut i = 0;
            while i < rules.len() {
                let mut r = &rules[i];
                if ordering.contains(&r[0]) && ordering.contains(&r[1]) && wrong {
                    let index1 = ordering.iter().position(|j| *j == r[0]).unwrap();
                    let index2 = ordering.iter().position(|j| *j == r[1]).unwrap();
                    if index2 < index1 {
                        let mut store = String::from("0");
                        store = ordering[index1].clone();
                        ordering[index1] = ordering[index2].clone();
                        ordering[index2] = store;
                        i = 0;
                    }
                }
                i+= 1;
            }
            if !wrong {
                result += ordering[ordering.len()/2].parse::<i32>().unwrap(); 
            } else {
                result2 += ordering[ordering.len()/2].parse::<i32>().unwrap(); 
            }
        }
    }
    println!("Part1 = {}", result);
    println!("Part2 = {}", result2);
}