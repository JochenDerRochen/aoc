use std::fs::read_to_string;

fn main() {
    let mut map = vec![0; 1000000];
    let mut brightness = vec![0; 1000000];
    let filename = "input.txt";
    for line in read_to_string(filename).unwrap().lines() {
        let l = line.to_string();
        let p = l.split(" ");
        let parts:Vec<&str> = p.collect();
        let mut startPos = (0, 0);
        let mut endPos = (0, 0);
        if parts[0] == "toggle" {
            let sPosString:Vec<&str> = parts[1].split(",").collect();
            startPos.0 = sPosString[0].parse::<i32>().unwrap();
            startPos.1 = sPosString[1].parse::<i32>().unwrap();
            let ePosString:Vec<&str> = parts[3].split(",").collect();
            endPos.0 =  ePosString[0].parse::<i32>().unwrap();
            endPos.1 =  ePosString[1].parse::<i32>().unwrap();
            for i in (startPos.0)..(endPos.0)+1 {
                for j in (startPos.1)..(endPos.1)+1 {
                    brightness[(j*1000 + i) as usize] += 2;
                    if map[(j*1000 + i) as usize] == 1 {
                        map[(j*1000 + i) as usize] = 0;
                    } else {
                        map[(j*1000 + i) as usize] = 1;
                    }
                }
            }
        }
        if parts[0] == "turn" {
            let typeString = parts[1];
            let action = if typeString == "on" {1} else {0};
            
            let sPosString:Vec<&str> = parts[2].split(",").collect();
            startPos.0 = sPosString[0].parse::<i32>().unwrap();
            startPos.1 = sPosString[1].parse::<i32>().unwrap();
            let ePosString:Vec<&str> = parts[4].split(",").collect();
            endPos.0 =  ePosString[0].parse::<i32>().unwrap();
            endPos.1 =  ePosString[1].parse::<i32>().unwrap();
            for i in (startPos.0)..(endPos.0)+1 {
                for j in (startPos.1)..(endPos.1)+1 {
                    if action == 0 {
                        brightness[(j*1000 + i) as usize] -= 1;
                        if brightness[(j*1000 + i) as usize] < 0 {
                            brightness[(j*1000 + i) as usize] = 0;
                        }
                    }
                    else if action == 1 {
                        brightness[(j*1000 + i) as usize] += 1;
                    }
                    if map[(j*1000 + i) as usize] == 1 && action == 0 {
                        map[(j*1000 + i) as usize] = 0;
                    } else if map[(j*1000 + i) as usize] == 0  && action == 1{
                        map[(j*1000 + i) as usize] = 1;
                    }
                }
            }
        } 
    }
    let mut count = 0;
    for i in map {
        if i == 1 {
            count += 1;
        }
    }

    let mut brightnessCount = 0;
    for i in brightness {
        brightnessCount += i;
    }
    println!("{}", count);
    println!("{}", brightnessCount);
}