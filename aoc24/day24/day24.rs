
use std::fs::read_to_string;
use std::collections::{HashMap, BTreeMap};
fn main() {
    let filename = "input.txt";
    let binding = read_to_string(filename).unwrap();
    let mut lines:Vec<&str> = binding.lines().collect();
    let mut map: BTreeMap<String, u16> = BTreeMap::new();
    let mut visitedLines = Vec::<usize>::new(); 
    while lines.len()-1 > visitedLines.len() {
        for i in 0..lines.len() {
            let curr = lines[i];
            let parts:Vec<&str> = curr.split(" ").collect();
            if parts.len() == 2 && !visitedLines.contains(&i) {
                let key:String = parts[0].chars().take(parts[0].len()-1).collect();
                let val = parts[1].parse::<u16>().unwrap();
                map.insert(key, val);
                visitedLines.push(i as usize);
            }
            if parts.len() == 5 && !visitedLines.contains(&i){
                if map.contains_key(parts[0]) && map.contains_key(parts[2]) {
                    let op = parts[1];
                    let mut val = 0;
                    if op == "AND" {
                        val = map[parts[0]] & map[parts[2]];
                    } else if op == "XOR" {
                        val = map[parts[0]] ^ map[parts[2]];
                    } else if op == "OR" {
                        val = map[parts[0]] | map[parts[2]];
                    }
                    map.insert(String::from(parts[4]), val);
                    visitedLines.push(i as usize);
                }
            } 
        }
    }
    let mut bin = String::from("");
    for key in map.keys() {
        if key.contains("z") {
            bin += &map[key].to_string();
        }
    }
    let rev = bin.chars().rev().collect::<String>();
    let intval = isize::from_str_radix(&rev, 2).unwrap();
    println!("{}", intval);
}