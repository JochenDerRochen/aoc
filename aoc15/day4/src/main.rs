use md5;

fn main() {
    let string = "ckczppom";
    let mut decimal = 0;
    while 1 == 1 {
        decimal += 1;
        let mut hashable = string.to_owned()+&decimal.to_string();
        let digest = md5::compute(hashable);
        let digestString = format!("{:x}", digest);
        let ch = &digestString[..6];
        if ch.chars().all(|x| x == '0') {
            println!("{}", decimal);
            break;
        }
    }
}