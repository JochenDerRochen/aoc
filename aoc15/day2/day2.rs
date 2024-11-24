use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let mut totalFeet = 0;
    let mut totalRibFeet = 0;
    for line in read_to_string(filename).unwrap().lines() {
        totalFeet += calculate(line.to_string());
        totalRibFeet += calculateRib(line.to_string());
    }
    println!("{}", totalFeet);
    println!("{}", totalRibFeet);
}

fn calculate(num:String) -> i64 {
    let mut nums = Vec::new();
    let mut feet = 0;
    let numbers = num.split('x');
    for n in numbers {
        nums.push(n.parse::<i64>().unwrap());
    }
    let l = nums[0];
    let w = nums[1];
    let h = nums[2];
    let sideO = l*w;
    let mut min = sideO;
    let sideT = w*h;
    if sideT < min {min = sideT;}
    let sideTh = h*l;
    if sideTh < min {min = sideTh;}
    feet += 2 * sideO + 2 * sideT + 2 * sideTh;
    feet += min;

    feet
    
}

fn calculateRib(num:String) -> i64 {
    let mut nums = Vec::new();
    let mut feet = 0;
    let numbers = num.split('x');
    for n in numbers {
        nums.push(n.parse::<i64>().unwrap());
    }

    nums.sort();
    
    feet += nums[0]*2 + nums[1]*2;
    feet += nums[0] * nums[1] * nums[2];

    feet
    
}
