fn main() {
    let input = include_str!("input.txt");
    let mut regA = 0;
    let mut regB = 0;
    let mut regC = 0;
    let mut pointer = 0;
    let mut program = Vec::<i32>::new();
    let mut output = Vec::<i32>::new();
    for line in input.lines() {
        if line.contains("Register A:") {
            let num:String = line.chars().skip(12).collect();
            regA = num.parse::<i32>().unwrap();
        } 
        if line.contains("Register B:") {
            let num:String = line.chars().skip(12).collect();
            regB = num.parse::<i32>().unwrap();
        } 
        if line.contains("Register C:") {
            let num:String = line.chars().skip(12).collect();
            regC = num.parse::<i32>().unwrap();
        } 
        if line.contains("Program:") {
            let instString:String = line.chars().skip(9).collect();
            let inst = instString.split(",");
            for i in inst {
                program.push(i.parse::<i32>().unwrap());
            }
        }
    }
    while pointer < program.len() { 
        let mut combo = program[pointer+1];
        let mut instruction = program[pointer];
        let mut num = 0;
        let mut jump = 2;
        //println!("{}", instruction);
        if combo <= 3 {
            num = combo;
        }
        if combo > 3 {
            if combo == 4 {
                num = regA;
            }
            if combo == 5 {
                num = regB;
            }
            if combo == 6 {
                num = regC;
            }
        }
        if instruction == 0{
            regA = regA / 2_i32.pow(num as u32);
        }
        if instruction == 1 {
            regB = regB ^ combo;
        }
        if instruction == 2 {
            regB = num % 8;
        }
        if instruction == 3 {
            if regA != 0 {
                jump = 0;
                pointer = combo as usize;
            }
        }
        if instruction == 4 {
            regB = regB ^ regC;
        }
        if instruction == 5 {
            output.push(num % 8);
        }
        if instruction == 6 {
            regB = regA / 2_i32.pow(num as u32);
        }
        if instruction == 7 {
            regC = regA / 2_i32.pow(num as u32);
        }
        pointer += jump;
    }
    println!("{:?}", output);
    let mut regA: i64 = 0;
    let mut regB: i64 = 0;
    let mut regC: i64 = 0;
    let mut pointer = 0;
    let mut program = Vec::<i64>::new();
    let mut output = Vec::<i64>::new();
    for line in input.lines() {
        if line.contains("Register B:") {
            let num:String = line.chars().skip(12).collect();
            regB = num.parse::<i64>().unwrap();
        } 
        if line.contains("Register C:") {
            let num:String = line.chars().skip(12).collect();
            regC = num.parse::<i64>().unwrap();
        } 
        if line.contains("Program:") {
            let instString:String = line.chars().skip(9).collect();
            let inst = instString.split(",");
            for i in inst {
                program.push(i.parse::<i64>().unwrap());
            }
        }
    }
    let mut inputs = (0..8).collect::<Vec<_>>();
    for n in 0..=program.len() {
        let mut next = vec![];
        for num in inputs {
            
        }
    }
    println!("{}", currentRegA-1);
}