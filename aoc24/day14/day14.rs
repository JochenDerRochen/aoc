struct Robot {
    posX: i32,
    posY: i32,
    velX: i32,
    velY: i32
}


fn main() {
    let input = include_str!("input.txt");
    let mut robots = Vec::<Robot>::new();
    let width = 101;
    let height = 103;
    for line in input.lines() {
        let posStart = line.find("=").unwrap();
        let velStart = line.find("v").unwrap();
        let t:String = line.chars().skip(posStart+1).take(velStart-posStart-2).collect();
        let poses = t.split(",").collect::<Vec<_>>();
        let posX = poses[0].parse::<i32>().unwrap();
        let posY = poses[1].parse::<i32>().unwrap();
        let vel:String = line.chars().skip(posStart+velStart+1).collect();
        let vels = vel.split(",").collect::<Vec<_>>();
        let velX = vels[0].parse::<i32>().unwrap();
        let velY = vels[1].parse::<i32>().unwrap();
        let r = Robot{
            posX: posX,
            posY: posY,
            velX: velX,
            velY: velY
        };
        robots.push(r);
    }
    let mut found = false;
    let mut moves = 0;
    while !found {
        moves += 1;
        let mut positions = Vec::<Vec<i32>>::new();
        for mut r in &mut robots {
            r.posX += r.velX;
            r.posY += r.velY;
            if r.posY < 0 {
                r.posY = height + r.posY;
            }
            if r.posY >= height {
                r.posY = r.posY - height;
            }
            if r.posX < 0 {
                r.posX = width + r.posX;
            }
            if r.posX >= width {
                r.posX = r.posX - width;
            }
            positions.push(vec![r.posX, r.posY]);
        }
        if moves == 100 {
            let mut result = 0;
            let middleX = width /2;
            let middleY = height/2;
            let mut leftU = 0;
            let mut rightU = 0;
            let mut leftL = 0;
            let mut rightL = 0;
            for r in &robots {
                if r.posX < middleX && r.posY < middleY {
                    leftU += 1;
                }
                if r.posX > middleX && r.posY < middleY {
                    rightU += 1;
                }
                if r.posX < middleX && r.posY >middleY {
                    leftL += 1;
                }
                if r.posX > middleX && r.posY > middleY {
                    rightL += 1;
                }
            }
            result = leftL * leftU * rightL * rightU;
            println!("Part1 = {}", result);
        } 
        positions.sort();
        let mut uniqueElements = positions.len();
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if positions[i] == positions[j] && i != j {
                    uniqueElements -= 1;
                    break;
                }
            }
        }
        if uniqueElements == positions.len() {
            found = true;
            println!("Part2 = {}", moves);
        }
    }
}