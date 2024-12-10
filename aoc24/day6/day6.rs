use std::collections::HashMap;
fn main() {
    let input = include_str!("input.txt");
    let mut visitedPos: Vec<Vec<i32>> = vec![vec![]];
    let mut positionOfObs = Vec::<Vec<i32>>::new();
    let mut orgPositionOfObs = Vec::<Vec<i32>>::new();
    let mut positionGuard: Vec<i32> = vec![0,0];
    let mut startingPos: Vec<i32> = vec![0,0];
    let vels = vec![vec![0,-1], vec![1,0], vec![0,1], vec![-1,0]];
    let mut currentVels = 0;
    let mut velGuard: &Vec<i32> = &vels[0];
    let mut lines = input.lines().collect::<Vec<_>>();
    let mut dim:i32 = 130;
    let mut iterator = 130;
    for i in 0..iterator {
        for j in 0..iterator {
            let mut current = '.';
            if let Some(ok) = lines[i].chars().nth(j).take() {
                current = ok;
            }
            if current == '#' {
                positionOfObs.push(vec![j as i32,i as i32]);
            }
            if current == '^' {
                positionGuard = vec![j as i32,i as i32];
                startingPos = vec![j as i32,i as i32];
            }
            visitedPos[0] = positionGuard.clone();
        }
    }
    
    orgPositionOfObs = positionOfObs.clone();
    positionGuard[0] += velGuard[0];
    positionGuard[1] += velGuard[1];

    while positionGuard[0] < dim && positionGuard[0] > -1 && positionGuard[1] < dim && positionGuard[1] > -1 {
        if positionOfObs.contains(&positionGuard) {
            currentVels += 1;
            positionGuard[0] -= velGuard[0];
            positionGuard[1] -= velGuard[1];
            velGuard = &vels[currentVels % 4];
        } else {
            if !visitedPos.contains(&positionGuard) {
                visitedPos.push(positionGuard.clone());
            }
        }
        positionGuard[0] += velGuard[0];
        positionGuard[1] += velGuard[1];
    }
    println!("{:?}", visitedPos.len());
    let mut loopCount = 0;
    for i in visitedPos {
            let mut currentHitted = HashMap::<Vec<i32>, i32>::new(); 
            positionOfObs = orgPositionOfObs.clone();
            if i != startingPos {
                positionOfObs.push(i); 
            }
            positionGuard = startingPos.clone();
            let mut foundLoop = false;
            currentVels = 0;
            velGuard = &vels[0];
            while !foundLoop && (positionGuard[0] < dim && positionGuard[0] > -1 && positionGuard[1] < dim && positionGuard[1] > -1) {
                if positionOfObs.contains(&positionGuard) {
                    if currentHitted.contains_key(&positionGuard) {
                        if currentHitted[&positionGuard] == ((currentVels % 4) as i32) {
                            foundLoop = true;
                        }
                    }
                    currentHitted.insert(positionGuard.clone(), (currentVels % 4) as i32);
                    positionGuard[0] -= velGuard[0];
                    positionGuard[1] -= velGuard[1];
                    currentVels += 1;
                    velGuard = &vels[currentVels % 4];
                }
                
                positionGuard[0] += velGuard[0];
                positionGuard[1] += velGuard[1];

            }
            if foundLoop {
                loopCount += 1;
            }
        
    }
    println!("{:?}", loopCount);

}
