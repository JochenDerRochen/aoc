
fn main() {
    let input = include_str!("input.txt");
    let mut decoded = Vec::<String>::new();
    let mut currentNumber = 0;
    for (idx, c) in input.chars().enumerate() {
        let length = c.to_digit(10).unwrap();
        if idx % 2 != 0 {
            for _ in 0..length {
                decoded.push(String::from("."));
            }
        } else {
            for _ in 0..length {
                let s: String = currentNumber.to_string();
                decoded.push(s);
            }
            currentNumber += 1;
        }
    }
    let mut leftPointer = 0;
    let mut rightPointer = decoded.len()-1;
    let mut result = 0;
    while leftPointer != rightPointer {
        if decoded[leftPointer] != "." {
            result += decoded[leftPointer].parse::<i64>().unwrap() * leftPointer as i64;
            leftPointer += 1;
        } else if decoded[leftPointer] == "." {
            if decoded[rightPointer] != "." {
                result += decoded[rightPointer].parse::<i64>().unwrap() * leftPointer as i64;
                leftPointer += 1;
            } 
            rightPointer -= 1;
        }
    }
    //result += decoded[leftPointer].parse::<i64>().unwrap() * leftPointer as i64;
    println!("{}", result);
    
    let mut leftPointer = 0;
    let mut rightPointer = decoded.len()-1;
    let mut result = 0;
    let mut spareCounter = 0;
    let mut blockCounter = 0;
    let mut foundBlock = false;
    let mut currentS = "";
    /*while rightPointer < decoded.len() {
        if decoded[leftPointer] != "." {
            //result += decoded[leftPointer].parse::<i64>().unwrap() * leftPointer as i64;
            if spareCounter > 0 {
                foundBlock = false;
                while !foundBlock{
                    blockCounter = 0;
                    while decoded[rightPointer] == "." {
                        rightPointer -= 1;
                    }
                    currentS = &decoded[rightPointer];
                    while decoded[rightPointer] == currentS {
                        blockCounter += 1;
                        rightPointer -= 1;
                    }
                    if blockCounter <= spareCounter {
                        foundBlock = true;
                        let value = decoded[((rightPointer as i64)+1) as usize].parse::<i64>().unwrap();
                        for i in 0..blockCounter {
                            //result += (leftPointer as i64) - 3 - (i as i64) * value ;
                            decoded[((leftPointer as i64) - (blockCounter+1) + (i as i64)) as usize] = value.to_string();
                            decoded[((rightPointer as i64) + 1 + (i as i64)) as usize] = ".".to_string();
                        }
                    }
                }
                println!("{:?}", decoded);
                rightPointer = decoded.len()-1;
                leftPointer = 0;
            } else {
                leftPointer += 1;
            }
            spareCounter = 0;
        } else if decoded[leftPointer] == "." {
            spareCounter += 1;
            leftPointer += 1;
        }
    }*/
    let mut leftPointer = 0;
    let mut rightPointer = decoded.len()-1;
    let mut result = 0;
    let mut spareCounter = 0;
    let mut blockCounter = 0;
    let mut foundBlock = false;
    let mut currentS = ".";
    while rightPointer > 0 {
        if decoded[rightPointer] == "." {
            rightPointer -= 1;
        } else if decoded[rightPointer] != "." {
            currentS = &decoded[rightPointer];
        }
        while decoded[rightPointer] == currentS && rightPointer > 0 {
            blockCounter += 1;
            rightPointer -= 1;
        }
        //println!("{}", currentS);
        //println!("{}", blockCounter);
        if blockCounter > 0 {
            leftPointer = 0;
            while leftPointer < rightPointer {
                spareCounter = 0;
                while decoded[leftPointer] != "." {
                    leftPointer += 1;
                }
                while decoded[leftPointer] == "." {
                    spareCounter += 1;
                    if leftPointer >= decoded.len()-1 {
                        break;
                    }
                    leftPointer += 1;
                }
                //println!("spare = {}", spareCounter);
                //println!("in loop = {} {}",leftPointer, rightPointer);
                if spareCounter >= blockCounter && blockCounter != 0 && spareCounter != 0 && leftPointer < rightPointer {
                    currentS = "";
                    //println!("{}", rightPointer); 
                    let value = decoded[((rightPointer as i64)+1) as usize].parse::<i64>().unwrap();
                    for i in 0..blockCounter{
                        decoded[((rightPointer as i64) + 1 + (i as i64)) as usize] = ".".to_string();
                        decoded[((leftPointer as i64) - (spareCounter) +  (i as i64)) as usize] = value.to_string();
                    }
                    spareCounter = 0;
                    blockCounter = 0;
                    break;
                } else {
                    leftPointer += 1;
                }
            }
        }
        blockCounter = 0;
    }
    for i in 0..decoded.len() {
        if decoded[i] != "."{
            let value = decoded[i].parse::<i64>().unwrap();
            result += i as i64 * value;
        }
    }
    //result += decoded[leftPointer].parse::<i64>().unwrap() * leftPointer as i64;
    println!("{}", result);
}