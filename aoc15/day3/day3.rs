use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let mut pos = vec![0, 0];
    let mut roboSanta = vec![0,0];
    let mut visited = Vec::new();
    visited.push(pos.clone());

    for line in read_to_string(filename).unwrap().lines() {
        for (idx,c) in line.chars().enumerate() {
            if c == '^' {
                if idx % 2 == 0 {
                    roboSanta[1] += 1;
                } else {
                    pos[1] += 1;
                }
            }
            else if c == 'v' {
                if idx % 2 == 0 {
                    roboSanta[1] -= 1;
                } else {
                    pos[1] -= 1;
                }
            }
            else if c == '>' {
                if idx % 2 == 0 {
                    roboSanta[0] += 1;
                } else {
                    pos[0] += 1;
                }
            }
            else if c == '<' {
                if idx % 2 == 0 {
                    roboSanta[0] -= 1;
                } else {
                    pos[0] -= 1;
                }
            }
            if !visited.contains(&pos) {
                visited.push(pos.clone());
            }
            if !visited.contains(&roboSanta) {
                visited.push(roboSanta.clone())
            }
        }
    }
    println!("{}", visited.len());
}