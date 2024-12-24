fn main() {
    let input = include_str!("input.txt");
    let mut result = 0;
    let mut insideObject = false;
    let mut dontSave = false;
    let mut objectIn = usize::MAX;
    for line in input.lines() {
        let mut current = String::from("");
        for (i,c) in line.chars().enumerate() {
            let s = String::from(c);
            if line.contains("red") && insideObject && line.find("red").unwrap() > objectIn {
                dontSave = true;
            }
            if c.is_numeric() || s == "-" {
                current += &s;
            } else {
                if s == "{" {
                    insideObject = true;
                    objectIn = i;
                }
                if s == "}" {
                    insideObject = false;
                    dontSave = false;
                    objectIn = usize::MAX;
                }
                if current.len() != 0 && !dontSave && i != line.len()-1 {
                    result += current.parse::<i32>().unwrap();
                    current = String::from("");
                } else {
                    current = String::from("");
                }
            }
        }
    }
    println!("{}", result);
}