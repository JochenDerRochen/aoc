use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let binding = read_to_string(filename).unwrap();
    let lines = binding.lines();
    let mut totalLength = 0;
    let mut codedLength = 0;
    let mut totalMagicLength = 0;
    for line in lines {
        totalLength += line.len() as i32;
        let mut stringLength = line.len() as i32;
        let mut magicLength = line.len() as i32;
        magicLength += 4;
        stringLength -=2;
        if line.contains("\\") {
            let mut chars = line.chars();
            for i in 0..line.len() {
                if line.chars().nth(i).unwrap() == '\\' {
                    if line.chars().nth(i + 1).unwrap() == 'x' {
                        if line.chars().nth(i+2).unwrap().is_ascii_hexdigit() && line.chars().nth(i+3).unwrap().is_ascii_hexdigit() {
                            stringLength -= 3;
                            magicLength += 1;
                        }
                    }
                    if (line.chars().nth(i+1).unwrap() == '\\') || ( line.chars().nth(i+1).unwrap() == '\"' && i + 1 < line.len()-2) {
                        stringLength -= 1;
                        magicLength += 2;
                    }
                }
            } 
        }
        codedLength += stringLength; 
        totalMagicLength += magicLength;
    }
    println!("{}", totalLength-codedLength);
    println!("{}", totalMagicLength - totalLength);
}