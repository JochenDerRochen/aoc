use std::fs::read_to_string;
use std::collections::HashMap;
fn main() {
    let filename = "input.txt";
    let binding = read_to_string(filename).unwrap();
    let mut lines:Vec<&str> = binding.lines().collect();
    let mut map: HashMap<String, u16> = HashMap::new();
    let mut visitedLines = Vec::<usize>::new(); 
    map.insert("b".to_string(), 16076);
    while !map.contains_key("a") {
        for i in 0..lines.len() {
            let curr = lines[i];
            let parts:Vec<&str> = curr.split(" ").collect();
            // must be a straight wire
            if parts.len() == 3 && !map.contains_key(parts[2]) {
                if parts[0].parse::<u16>().is_ok() {
                    let val = parts[0].parse::<u16>().unwrap();
                    let key = parts[2];
                    map.insert(key.to_string(), val);
                    visitedLines.push(i as usize)
                } else if map.contains_key(parts[0]) {
                    let val = map[parts[0]];
                    let key = parts[2];
                    map.insert(key.to_string(), val);
                    visitedLines.push(i as usize)
                }
            }
            //must be a not
            if parts.len() == 4 {
                if map.contains_key(parts[1]) && !map.contains_key(parts[3]){
                    let val = !map[parts[1]]; 
                    let key = parts[3].to_string();
                    map.insert(key, val);
                    visitedLines.push(i as usize);
                }
            }
            if parts.len() == 5 {
                if !map.contains_key(parts[4]) {
                    let key = parts[4].to_string();
                    //must be R or L shift
                    if parts[2].parse::<u16>().is_ok() && map.contains_key(parts[0]) {
                        //println!("{}", parts[2]);
                        let times = parts[2].parse::<u16>().unwrap();
                        if parts[1] == "RSHIFT" {
                            let val = map[parts[0]] >> times;
                            map.insert(key, val);
                            visitedLines.push(i as usize);
                        } else {
                            let val = map[parts[0]] << times;
                            map.insert(key, val);
                            visitedLines.push(i as usize);
                        }
                    } else {
                        if map.contains_key(parts[2]) && map.contains_key(parts[0]) {
                            if parts[1] == "OR" {
                                let val = map[parts[0]] | map[parts[2]];
                                map.insert(key, val);
                                visitedLines.push(i as usize);
                            } else {
                                let val = map[parts[0]] & map[parts[2]];
                                map.insert(key, val);
                                visitedLines.push(i as usize);
                            }
                        } else if parts[0].parse::<u16>().is_ok() && map.contains_key(parts[2]) {
                            if parts[1] == "OR" {
                                let val = parts[0].parse::<u16>().unwrap() | map[parts[2]];
                                map.insert(key, val);
                                visitedLines.push(i as usize);
                            } else {
                                let val = 1 & map[parts[2]];
                                map.insert(key, val);
                                visitedLines.push(i as usize);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", map["a"]);
}