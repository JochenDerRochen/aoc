use std::fs::read_to_string;
fn main() {
    let mut niceStrings = 0;
    let mut veryNiceStrings = 0;
    let filename = "input.txt";
    for line in read_to_string(filename).unwrap().lines() {
        let l = line.to_string();
        if checkForVowels(l.clone()) && checkForDouble(l.clone()) && checkForFalseStrings(l.clone()) {
            niceStrings += 1;
        }
        if checkForPali(l.clone()) && checkForPair(l.clone()) {
            veryNiceStrings += 1 
        }
    }
    println!("{}", niceStrings);
    println!("{}", veryNiceStrings);
}

fn checkForVowels(s: String) -> bool {
    let mut vowelCounter = 0;
    for char in s.chars() {
        if char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' {
            vowelCounter += 1;
        }
    }

    vowelCounter >= 3
}

fn checkForDouble(s:String) -> bool {
    let mut chars = s.chars();
    for i in 0..(s.len()-1) {
        if s.as_bytes()[i] == s.as_bytes()[i+1] {
            return true;
        }
    }

    false
}

fn checkForPali(s:String) -> bool {
    let mut chars = s.chars();
    for i in 0..(s.len()-2) {
        if s.as_bytes()[i] == s.as_bytes()[i+2] && s.as_bytes()[i+2] != s.as_bytes()[i+1]{
            return true;
        }
    }
    false
}

fn checkForPair(s:String) -> bool {
    let mut pair = vec!{0,0};
    for i in 0..(s.len()-1) {
        pair[0] = s.as_bytes()[i];
        pair[1] = s.as_bytes()[i+1];
        for j in 0..(s.len()-1) {
            if j != i && j != i+1 {
                if pair[0] == s.as_bytes()[j] && pair[1] == s.as_bytes()[j+1] {
                    return true;
                }

            }
        }
    }

    false
}

fn checkForFalseStrings(s:String) -> bool {
    !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy")
}