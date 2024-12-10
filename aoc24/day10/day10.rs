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
    let mut result = 0;
    const dim:i32 = 54;
    let mut map = [[0u32; dim as usize]; dim as usize];
    let mut check = Vec::<Point>::new();
    let mut startings = Vec::<Point>::new();
    let mut edges = Vec::<Vec<Point>>::new();
    let mut endings = Vec::<Point>::new();
    for (idx, line) in lines.enumerate() {
        for (j, char) in line.chars().enumerate() {
            let dig = char.to_digit(10).unwrap();
            if dig == 0 {
                let mut p = Point{
                    x: j as i32,
                    y: idx as i32
                };
                check.push(p);
                startings.push(p);
            } else if dig == 9 {
                let mut p = Point {
                    x: j as i32,
                    y: idx as i32
                };
                endings.push(p);
            }
            map[idx][j] = dig;
        }
    }
    for start in startings {
        let mut check = Vec::<Point>::new();
        check.push(start);
        let mut seen = Vec::<Point>::new();
        while check.len() != 0 {
            let current = check.pop().unwrap();
            let x = current.x as usize;
            //println!("{:?}", current);
            let y = current.y as usize;
            if endings.contains(&current) && !seen.contains(&current) { 
                result += 1;
                //seen.push(current); // remove for part 2
            }
            else {
                if current.y + 1 < dim {
                    let x = current.x as usize;
                    let y = current.y as usize;
                    if map[y+1][x] == map[y][x] + 1 {
                        let p = Point{
                            x: current.x,
                            y: current.y+1
                        };
                        check.push(p);
                    }
                }
                if current.y - 1 >= 0 {
                    let x = current.x as usize;
                    let y = current.y as usize;
                    if map[y-1][x] == map[y][x] + 1 {
                        let p = Point{
                            x: current.x,
                            y: current.y-1
                        };
                        check.push(p);
                    }
                }
                if current.x + 1 < dim {
                    let x = current.x as usize;
                    let y = current.y as usize;
                    if map[y][x+1] == map[y][x] + 1{
                        let p = Point{
                            x: current.x+1,
                            y: current.y
                        };
                        check.push(p);
                    }
                }
                if current.x - 1 >= 0 {
                    let x = current.x as usize;
                    let y = current.y as usize;
                    if map[y][x-1] == map[y][x] + 1 {
                        let p = Point{
                            x: current.x-1,
                            y: current.y
                        };
                        check.push(p);
                    }
                }
            }
        }
    }
    
    
    //println!("{:?}", edges);
    println!("{}", result);
}