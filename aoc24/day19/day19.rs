use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    let mut availableTowels = Vec::<String>::new();
    let mut creatableTowels = Vec::<String>::new();
    for line in input.lines() {
        if line.contains(",") {
            let removed = line.replace(" ", "");
            let aTowels = removed.split(",");
            for t in aTowels {
                availableTowels.push(String::from(t));
            }
        }
        if line.len() > 0 && !line.contains(",") {
            creatableTowels.push(String::from(line));
        }
    }
    let mut usableTowels = Vec::<String>::new();
    let mut matched = 0;
    for t in &creatableTowels {
        println!("{}", t);
        if isDoable(t.clone(), &availableTowels) {
            matched += 1;
        }
    }
    println!("{}", matched);
}
fn isDoable(t: String, towels:&Vec::<String>) -> bool {
    let mut pieces = VecDeque::new();
    pieces.push_back(t.clone());
    while pieces.len() != 0 {
        let current = pieces.pop_front().unwrap();
        for towel in towels {
            let check: String= current.chars().take(towel.len()).collect();
            if check == *towel {
                if *current == *towel {
                    return true;
                } else {
                    let adding:String = current.chars().skip(towel.len()).collect();
                    if !pieces.contains(&adding) {
                        pieces.push_front(adding.clone());
                    }
                }
            }
        }
    }
    return false;
}