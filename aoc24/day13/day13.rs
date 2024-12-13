fn main() {

    let input = include_str!("input.txt");
    let mut A = vec![vec![0.0; 2]; 2];
    let mut b = vec![0.0,0.0];
    let mut b1 = vec![0.0, 0.0];
    let mut result = 0;
    let mut result1 = 0;
    for line in input.lines() {
        if line.contains("Button A") {
            let x:String = line.chars().skip(12).take(2).collect();
            let y:String = line.chars().skip(18).take(2).collect();
            A[0][0] = x.parse::<f64>().unwrap();
            A[0][1] = y.parse::<f64>().unwrap();
        }
        if line.contains("Button B") {
            let x:String = line.chars().skip(12).take(2).collect();
            let y:String = line.chars().skip(18).take(2).collect();
            A[1][0] = x.parse::<f64>().unwrap();
            A[1][1] = y.parse::<f64>().unwrap();
        }
        if line.contains("Prize:") {
            let comma = line.find(",").unwrap();
            let X:String = line.chars().skip(9).take(comma - 9).collect();
            let Y:String = line.chars().skip(comma+4).collect();
            b[0] = X.parse::<f64>().unwrap();
            b[1] = Y.parse::<f64>().unwrap();
            b1[0] = b[0] + 10000000000000.0;
            b1[1] = b[1] + 10000000000000.0;
            let factor = 1.0/(A[0][0]*A[1][1] - A[0][1]*A[1][0]);
            let mut invA = vec![vec![0.0; 2]; 2];
            invA[0][0] = factor * A[1][1];
            invA[1][0] = factor * -A[1][0];
            invA[0][1] = factor * -A[0][1];
            invA[1][1] = factor * A[0][0];
            let aSol = invA[0][0] * b[0] + invA[1][0] * b[1];
            let bSol = invA[0][1] * b[0] + invA[1][1] * b[1];
            let aSol1 = invA[0][0] * b1[0] + invA[1][0] * b1[1];
            let bSol1 = invA[0][1] * b1[0] + invA[1][1] * b1[1];
            if (bSol.fract() < 0.001 || bSol.fract() > 0.999) && (aSol.fract() < 0.01 || aSol.fract() > 0.999){
                let a = aSol.round() as i64;
                let b = bSol.round() as i64;
                result += a*3 + b;
            }
            if (bSol1.fract() < 0.001 || bSol1.fract() > 0.999) && (aSol1.fract() < 0.01 || aSol1.fract() > 0.999){
                let a = aSol1.round() as i64;
                let b = bSol1.round() as i64;
                result1 += a*3 + b;
            }
        }
    }
    println!("Part1 = {}", result);
    println!("Part2 = {}", result1);
}