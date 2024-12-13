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
    const dim:i32 = 140;
    let mut result1 = 0;
    let mut map = vec![vec!['.'; dim as usize];dim as usize];
    let mut seen = Vec::<Point>::new();
    let lines = input.lines();
    let mut result = 0;
    for (idx, line) in lines.enumerate() {
        for (j, char) in line.chars().enumerate() {
            map[idx][j] = char;
        }
    }
    for i in 0..dim {
        for j in 0..dim {
            let mut stack = Vec::<Point>::new();
            let p = Point{x:j as i32, y: i as i32};
            if !seen.contains(&p) {
                let mut filling = Vec::<Point>::new();
                stack.push(p);
                filling.push(p);
                while stack.len() != 0 {
                    let current = stack.pop().unwrap();
                    let x = current.x as usize;
                    let y = current.y as usize;
                    if current.y + 1 < dim {
                        let p = Point{x:(x) as i32, y:(y+1) as i32};
                        if map[y][x] == map[y+1][x] && !filling.contains(&p)  {
                            stack.push(p);
                            filling.push(p);
                        }
                    }
                    if current.x + 1 < dim {
                        let p = Point{x: (x+1) as i32, y:y as i32};
                        if map[y][x] == map[y][x+1] && !filling.contains(&p) {
                            stack.push(p);
                            filling.push(p);
                        }
                    }

                    if current.x - 1 >= 0 {
                        let p = Point{x:(x-1) as i32, y:(y as i32)};
                        if map[y][x] == map[y][x-1] && !filling.contains(&p) {
                            stack.push(p);
                            filling.push(p);
                        }
                    }
                    if current.y - 1 >= 0{
                        let p = Point{x: x as i32, y:(y-1) as i32};
                        if map[y][x] == map[y-1][x] && !filling.contains(&p) {
                            stack.push(p);
                            filling.push(p);
                        }
                    }
                }
                let mut perimeter = 0;
                let mut area = filling.len();
                for pot in &filling {
                    let x = pot.x as usize;
                    let y = pot.y as usize;
                    if pot.x - 1 >= 0 {
                        if map[y][x] != map[y][x-1] {
                          perimeter +=1  
                        }
                    } else {
                        perimeter += 1;
                    }
                    if pot.y - 1 >= 0 {
                        if map[y][x] != map[y-1][x] {
                          perimeter +=1  
                        }
                    } else {
                        perimeter += 1;
                    }
                    if pot.x + 1 < dim {
                        if map[y][x] != map[y][x+1] {
                            perimeter += 1;
                        }
                    } else {
                        perimeter += 1;
                    }
                    
                    if pot.y + 1< dim {
                        if map[y][x] != map[y+1][x] {
                            perimeter += 1;
                        }
                    } else {
                        perimeter += 1;
                    }
                }

                let mut perimeter1 = 0;
                for pot in &filling {
                    let x = pot.x as usize;
                    let y = pot.y as usize;
                    if pot.x + 1 < dim && pot.y-1 >= 0 {
                        if map[y][x] != map[y-1][x+1] && map[y][x] == map[y][x+1] && map[y][x] == map[y-1][x]{
                            perimeter1 += 1;
                        }
                        if map[y][x] != map[y][x+1] && map[y][x] != map[y-1][x]{
                            perimeter1 += 1;
                        }
                    } 
                    if pot.x - 1 >= 0 && pot.y+1 < dim {
                        if map[y][x] != map[y+1][x-1] && map[y][x] == map[y][x-1] && map[y][x] == map[y+1][x]{
                            perimeter1 += 1;
                        }
                        if map[y][x] != map[y][x-1] && map[y][x] != map[y+1][x]{
                            perimeter1 += 1;
                        }
                    } 
                    if pot.x + 1 < dim && pot.y+1 < dim  {
                        if map[y][x] != map[y+1][x+1] && map[y][x] == map[y][x+1] && map[y][x] == map[y+1][x]{
                            perimeter1 += 1;
                        }
                        if map[y][x] != map[y][x+1] && map[y][x] != map[y+1][x]{
                            perimeter1 += 1;
                        }
                    }
                    if pot.x - 1 >= 0 && pot.y-1 >= 0  {
                        if map[y][x] != map[y-1][x-1] && map[y][x] == map[y][x-1] && map[y][x] == map[y-1][x]{
                            perimeter1 += 1;
                        }
                        if map[y][x] != map[y][x-1] && map[y][x] != map[y-1][x]{
                            perimeter1 += 1;
                        }
                    }
                    if pot.x -1 < 0 && pot.y-1 < 0 { 
                        perimeter1 += 1;
                    }
                    if pot.x +1 >= dim && pot.y-1 < 0 { 
                        perimeter1 += 1;
                    }
                    if pot.x -1 < 0 && pot.y +1 >= dim {
                        perimeter1 += 1;
                    }
                    if pot.x + 1 >= dim && pot.y+1 >= dim {
                        perimeter1 += 1;
                    }

                    if pot.x + 1 >= dim && pot.y-1 >= 0 {
                        if map[y][x] != map[y-1][x]{
                            perimeter1 += 1;
                        }
                    }
                    if pot.x + 1 >= dim && pot.y+1 < dim {
                        if map[y][x] != map[y+1][x]{
                            perimeter1 += 1;
                        }
                    }
                    if pot.x - 1 < 0 && pot.y-1 >= 0 {
                        if map[y][x] != map[y-1][x]{
                            perimeter1 += 1;
                        }
                    }
                    if pot.x - 1 < 0 && pot.y+1 < dim {
                        if map[y][x] != map[y+1][x]{
                            perimeter1 += 1;
                        }
                    }

                    if pot.y + 1 >= dim && pot.x-1 >= 0 {
                        if map[y][x] != map[y][x-1]{
                            perimeter1 += 1;
                        }
                    }
                    if pot.y + 1 >= dim && pot.x+1 < dim {
                        if map[y][x] != map[y][x+1]{
                            perimeter1 += 1;
                        }
                    }
                    if pot.y - 1 < 0 && pot.x-1 >= 0 {
                        if map[y][x] != map[y][x-1]{
                            perimeter1 += 1;
                        }
                    }
                    if pot.y - 1 < 0 && pot.x+1 < dim {
                        if map[y][x] != map[y][x+1]{
                            perimeter1 += 1;
                        }
                    }
                }
                result += perimeter*area;
                result1 += perimeter1*area;
                seen.append( &mut filling);
            }
        
        }
    }
    println!("{}", result);
    println!("{}", result1);
}