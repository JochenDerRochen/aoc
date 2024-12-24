fn main() {
    let mut input = String::from("1113222113");
    let mut result = String::from("");
    for i in 0..50 {
        let mut current:String = input.chars().take(1).collect();
        input = input.chars().skip(1).collect();
        result = String::from("");
        let mut counter = 1;
        for c in input.chars() {
            if String::from(c) == current {
                counter += 1;
            } else {
                result += &counter.to_string();
                result += &String::from(current);
                current = String::from(c);
                counter = 1;
            }
        }
        result += &counter.to_string();
        result += &String::from(current);
        input = result.clone();
    }
    println!("{}", result.len());
}