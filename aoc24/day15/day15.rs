use std::fmt;

#[derive(PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

impl Copy for Point { }

impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}

fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let mut boxes = Vec::<Point>::new();
    let mut bigBoxes = Vec::<Point>::new();
    let mut bigObstacles = Vec::<Point>::new();
    let mut robot = Point{x:0, y:0};
    let mut bigRobot = Point{x:0, y:0};
    let mut actions = Vec::<char>::new();
    let mut obstacles = Vec::<Point>::new();
    for (i, line) in lines.enumerate() {
        if line.contains('#') {
            for (j, char) in line.chars().enumerate() {
                if char == '@' {
                    robot = Point{x: j as i32, y:i as i32};
                    bigRobot = Point{x: (j*2) as i32, y: (i) as i32}
                }
                if char == 'O' {
                    boxes.push(Point{x: j as i32, y: i as i32});
                    bigBoxes.push(Point{x:(j*2) as i32, y: i as i32});
                }
                if char == '#' {
                    obstacles.push(Point{x: j as i32, y: i as i32});
                    bigObstacles.push(Point{x: (j*2) as i32, y: (i) as i32});
                    bigObstacles.push(Point{x: (j*2+1) as i32, y: (i) as i32});
                }
            }
        }
        if line.contains('<') || line.contains('>') || line.contains('v') || line.contains('^') {
            for char in line.chars() {
                actions.push(char);
            }
        }
    }

    for action in &actions {
        if *action == '^' {
            let newPos = Point{x: robot.x, y: robot.y-1};
            if !obstacles.contains(&newPos) {
                if boxes.contains(&newPos) {
                    let mut numBoxes = 0;
                    let mut pos = newPos;
                    let mut boxIndexes = Vec::<usize>::new();
                    while boxes.contains(&pos) {
                        let index = boxes.iter().position(|&r| r == pos).unwrap();
                        boxIndexes.push(index);
                        pos = Point{x: pos.x, y: pos.y-1};
                    }
                    if !obstacles.contains(&pos) {
                        for i in boxIndexes {
                            boxes[i] = Point{x: boxes[i].x, y: boxes[i].y-1}
                        }
                        robot = newPos;
                    }
                } else {
                    robot = newPos;
                }
            }
        }
        if *action == 'v' {
            let newPos = Point{x: robot.x, y: robot.y+1};
            if !obstacles.contains(&newPos) {
                if boxes.contains(&newPos) {
                    let mut numBoxes = 0;
                    let mut pos = newPos;
                    let mut boxIndexes = Vec::<usize>::new();
                    while boxes.contains(&pos) {
                        let index = boxes.iter().position(|&r| r == pos).unwrap();
                        boxIndexes.push(index);
                        pos = Point{x: pos.x, y: pos.y+1};
                    }
                    if !obstacles.contains(&pos) {
                        for i in boxIndexes {
                            boxes[i] = Point{x: boxes[i].x, y: boxes[i].y+1}
                        }
                        robot = newPos;
                    }
                } else {
                    robot = newPos;
                }
            }
        }
        if *action == '>' {
            let newPos = Point{x: robot.x+1, y: robot.y};
            if !obstacles.contains(&newPos) {
                if boxes.contains(&newPos) {
                    let mut numBoxes = 0;
                    let mut pos = newPos;
                    let mut boxIndexes = Vec::<usize>::new();
                    while boxes.contains(&pos) {
                        let index = boxes.iter().position(|&r| r == pos).unwrap();
                        boxIndexes.push(index);
                        pos = Point{x: pos.x+1, y: pos.y};
                    }
                    if !obstacles.contains(&pos) {
                        for i in boxIndexes {
                            boxes[i] = Point{x: boxes[i].x+1, y: boxes[i].y}
                        }
                        robot = newPos;
                    }
                } else {
                    robot = newPos;
                }
            }
        }
        if *action == '<' {
            let newPos = Point{x: robot.x-1, y: robot.y};
            if !obstacles.contains(&newPos) {
                if boxes.contains(&newPos) {
                    let mut numBoxes = 0;
                    let mut pos = newPos;
                    let mut boxIndexes = Vec::<usize>::new();
                    while boxes.contains(&pos) {
                        let index = boxes.iter().position(|&r| r == pos).unwrap();
                        boxIndexes.push(index);
                        pos = Point{x: pos.x-1, y: pos.y};
                    }
                    if !obstacles.contains(&pos) {
                        for i in boxIndexes {
                            boxes[i] = Point{x: boxes[i].x-1, y: boxes[i].y}
                        }
                        robot = newPos;
                    }
                } else {
                    robot = newPos;
                }
            }
        }
    }
    let mut result = 0;
    for b in &boxes {
        result += b.y * 100 + b.x;
    }
    println!("{}", result);


    for action in &actions {
        if *action == '^' {
            let newPos = Point{x: bigRobot.x, y: bigRobot.y-1};
            let boxCheck = Point{x: bigRobot.x - 1, y: bigRobot.y-1};
            if !bigObstacles.contains(&newPos) {
                if bigBoxes.contains(&newPos) || bigBoxes.contains(&boxCheck) {
                    let mut numBoxes = 0;
                    let mut boxIndexes = Vec::<usize>::new();
                    let mut checkBoxes = Vec::<Point>::new();
                    if bigBoxes.contains(&newPos) {
                        checkBoxes.push(newPos);
                    } else {
                        checkBoxes.push(boxCheck);
                    }
                    while checkBoxes.len() != 0 {
                        let currentBox = checkBoxes.pop().unwrap();
                        let index = bigBoxes.iter().position(|&r| r == currentBox).unwrap();
                        if !boxIndexes.contains(&index) {
                            boxIndexes.push(index)
                        }
                        let left = Point{x: currentBox.x-1, y: currentBox.y-1};
                        let onTop = Point{x: currentBox.x, y: currentBox.y-1};
                        let right = Point{x: currentBox.x+1, y: currentBox.y-1};
                        if bigBoxes.contains(&left) {
                            if !checkBoxes.contains(&left){
                                checkBoxes.push(left);
                            }
                        }
                        if bigBoxes.contains(&onTop) {
                            if !checkBoxes.contains(&onTop){
                                checkBoxes.push(onTop);
                            }
                        }
                        if bigBoxes.contains(&right) {
                            if !checkBoxes.contains(&right) {
                                checkBoxes.push(right);
                            }
                        }
                    }
                    let mut movable = true;
                    for index in &boxIndexes {
                        let boxPos = bigBoxes[*index];
                        let boxRight = Point{x: boxPos.x+1, y: boxPos.y-1};
                        let boxTop = Point{x:boxPos.x, y: boxPos.y-1};
                        movable = !bigObstacles.contains(&boxRight) && !bigObstacles.contains(&boxTop);
                        if !movable {
                            break;
                        }
                    }
                    if movable {
                        for i in boxIndexes {
                            bigBoxes[i] = Point{x: bigBoxes[i].x, y: bigBoxes[i].y-1}   
                        }
                        bigRobot = newPos;
                    }
                } else {
                    bigRobot = newPos;
                }
            }
        }
        if *action == 'v' {
            let newPos = Point{x: bigRobot.x, y: bigRobot.y+1};
            let rightCheck = Point{x: bigRobot.x - 1, y: bigRobot.y+1};
            let boxCheck = Point{x: bigRobot.x - 1, y: bigRobot.y+1};
            if !bigObstacles.contains(&newPos) {
                if bigBoxes.contains(&newPos) || bigBoxes.contains(&boxCheck) {
                    let mut numBoxes = 0;
                    let mut boxIndexes = Vec::<usize>::new();
                    let mut checkBoxes = Vec::<Point>::new();
                    if bigBoxes.contains(&newPos) {
                        checkBoxes.push(newPos);
                    } else {
                        checkBoxes.push(boxCheck);
                    }
                    while checkBoxes.len() != 0 {
                        let currentBox = checkBoxes.pop().unwrap();
                        let index = bigBoxes.iter().position(|&r| r == currentBox).unwrap();
                        if !boxIndexes.contains(&index) {
                            boxIndexes.push(index)
                        }
                        let left = Point{x: currentBox.x-1, y: currentBox.y+1};
                        let onTop = Point{x: currentBox.x, y: currentBox.y+1};
                        let right = Point{x: currentBox.x+1, y: currentBox.y+1};
                        if bigBoxes.contains(&left) {
                            if !checkBoxes.contains(&left){
                                checkBoxes.push(left);
                            }
                        }
                        if bigBoxes.contains(&onTop) {
                            if !checkBoxes.contains(&onTop){
                                checkBoxes.push(onTop);
                            }
                        }
                        if bigBoxes.contains(&right) {
                            if !checkBoxes.contains(&right) {
                                checkBoxes.push(right);
                            }
                        }
                    }
                    let mut movable = true;
                    for index in &boxIndexes {
                        let boxPos = bigBoxes[*index];
                        let boxRight = Point{x: boxPos.x+1, y: boxPos.y+1};
                        let boxTop = Point{x:boxPos.x, y: boxPos.y+1};
                        movable = !bigObstacles.contains(&boxRight) && !bigObstacles.contains(&boxTop);
                        if !movable {
                            break;
                        }
                    }
                    if movable {
                        for i in boxIndexes {
                            bigBoxes[i] = Point{x: bigBoxes[i].x, y: bigBoxes[i].y+1}   
                        }
                        bigRobot = newPos;
                    }
                } else {
                    bigRobot = newPos;
                }
            } 
        }
        if *action == '>' {
            let newPos = Point{x: bigRobot.x+1, y: bigRobot.y};
            if !bigObstacles.contains(&newPos) {
                if bigBoxes.contains(&newPos) {
                    let mut numBoxes = 0;
                    let mut pos = newPos;
                    let mut boxIndexes = Vec::<usize>::new();
                    while bigBoxes.contains(&pos) {
                        let index = bigBoxes.iter().position(|&r| r == pos).unwrap();
                        boxIndexes.push(index);
                        pos = Point{x: pos.x+2, y: pos.y};
                    }
                    if !bigObstacles.contains(&pos) {
                        for i in boxIndexes {
                            bigBoxes[i] = Point{x: bigBoxes[i].x+1, y: bigBoxes[i].y}
                        }
                        bigRobot = newPos;
                    }
                } else {
                    bigRobot = newPos;
                }
            }
        }
        if *action == '<' {
            let newPos = Point{x: bigRobot.x-1, y: bigRobot.y};
            let boxCheck = Point{x: bigRobot.x-2, y: bigRobot.y};
            if !bigObstacles.contains(&newPos) {
                if bigBoxes.contains(&boxCheck) {
                    let mut pos = newPos;
                    let mut posBox = boxCheck;
                    let mut boxIndexes = Vec::<usize>::new();
                    while bigBoxes.contains(&posBox) {
                        let index = bigBoxes.iter().position(|&r| r == posBox).unwrap();
                        boxIndexes.push(index);
                        pos = Point{x: posBox.x-1, y: posBox.y};
                        posBox = Point{x:posBox.x-2, y:posBox.y};
                    }
                    if !bigObstacles.contains(&pos) {
                        for i in boxIndexes {
                            bigBoxes[i] = Point{x: bigBoxes[i].x-1, y: bigBoxes[i].y};
                        }
                        bigRobot = newPos;
                    }
                } 
                else {
                    bigRobot = newPos;
                }
            }
        }
    }
    let mut result = 0;
    for b in &bigBoxes {
        result += b.y * 100 + b.x;
    }
    println!("{}", result);
}