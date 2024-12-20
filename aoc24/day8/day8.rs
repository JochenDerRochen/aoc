use std::fmt;
use std::collections::HashMap;

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
    let mut nodes = HashMap::<char, Vec<Point>>::new();
    let dim = 50;
    for (i, line) in lines.enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char != '.' {
                let p = Point{x: j as i32, y:i as i32};
                if nodes.contains_key(&char) {
                    nodes.get_mut(&char).unwrap().push(p);
                } else {
                    nodes.insert(char, vec![p]);
                } 
            } 
        }
    }
    let mut result = 0;
    let mut seen = Vec::<Point>::new();
    let mut antinodes = Vec::<Point>::new();
    for key in nodes.keys() {
        let antennas = nodes[key].clone();
        let mut antinodes1 = Vec::<Point>::new();
        for antenna1 in &antennas {
            for antenna2 in &antennas {
                if *antenna1 != *antenna2 && !seen.contains(antenna2) {
                    let distX = (antenna1.x - antenna2.x).abs();
                    let distY = (antenna1.y - antenna2.y).abs();
                    if antenna1.x - antenna2.x < 0 && antenna1.y - antenna2.y < 0{
                        antinodes1.push(Point{x: antenna1.x - distX, y: antenna1.y - distY});
                        antinodes1.push(Point{x: antenna2.x + distX, y: antenna2.y + distY});
                    }
                    if antenna1.x - antenna2.x < 0 && antenna1.y - antenna2.y > 0{
                        antinodes1.push(Point{x: antenna1.x - distX, y: antenna1.y + distY});
                        antinodes1.push(Point{x: antenna2.x + distX, y: antenna2.y - distY});
                    }
                    if antenna1.x - antenna2.x > 0 && antenna1.y - antenna2.y < 0{
                        antinodes1.push(Point{x: antenna1.x + distX, y: antenna1.y - distY});
                        antinodes1.push(Point{x: antenna2.x - distX, y: antenna2.y + distY});
                    }
                    if antenna1.x - antenna2.x > 0 && antenna1.y - antenna2.y > 0 {
                        antinodes1.push(Point{x: antenna1.x + distX, y: antenna1.y + distY});
                        antinodes1.push(Point{x: antenna2.x - distX, y: antenna2.y - distY});
                    }
                }
            }
            seen.push(*antenna1)
        }
        for antinode in antinodes1 {
            if antinode.x < dim && antinode.x >= 0 && antinode.y < dim && antinode.y >= 0 && !antinodes.contains(&antinode) {
                antinodes.push(antinode);
            }
        }
    }
    println!("{:?}", antinodes.len());
    let mut result = 0;
    let mut seen = Vec::<Point>::new();
    let mut antinodes = Vec::<Point>::new();
    for key in nodes.keys() {
        let antennas = nodes[key].clone();
        let mut antinodes1 = Vec::<Point>::new();
        for antenna1 in &antennas {
            for antenna2 in &antennas {
                let mut antiCount = 0;
                if *antenna1 != *antenna2 && !seen.contains(antenna2) {
                    let distX = (antenna1.x - antenna2.x).abs();
                    let distY = (antenna1.y - antenna2.y).abs();
                    if antenna1.x - antenna2.x < 0 && antenna1.y - antenna2.y < 0{
                        for i in 1..50 {
                            let p1 = Point{x:antenna1.x - distX*i, y: antenna1.y - distY*i};
                            if p1.x < dim && p1.x >= 0 && p1.y < dim && p1.y >= 0 {
                                antiCount += 1;
                                antinodes1.push(p1);
                            }
                            let p2 = Point{x:antenna2.x + distX*i, y: antenna2.y + distY*i};
                            if p2.x < dim && p2.x >= 0 && p2.y < dim && p2.y >= 0 {
                                antiCount += 1;
                                antinodes1.push(p2);
                            }
                        }
                    }
                    if antenna1.x - antenna2.x < 0 && antenna1.y - antenna2.y > 0{
                        for i in 1..50 {
                            let p1 = Point{x:antenna1.x - distX*i, y: antenna1.y + distY*i};
                            if p1.x < dim && p1.x >= 0 && p1.y < dim && p1.y >= 0 {
                                antiCount += 1;
                                antinodes1.push(p1);
                            }
                            let p2 = Point{x:antenna2.x + distX*i, y: antenna2.y - distY*i};
                            if p2.x < dim && p2.x >= 0 && p2.y < dim && p2.y >= 0 {
                                antiCount += 1;
                                antinodes1.push(p2);
                            }
                        }
                    }
                    if antenna1.x - antenna2.x > 0 && antenna1.y - antenna2.y < 0{
                        for i in 1..50 {
                            let p1 = Point{x:antenna1.x + distX*i, y: antenna1.y - distY*i};
                            if p1.x < dim && p1.x >= 0 && p1.y < dim && p1.y >= 0 {
                                antiCount += 1;
                                antinodes1.push(p1);
                            }
                            let p2 = Point{x:antenna2.x - distX*i, y: antenna2.y + distY*i};
                            if p2.x < dim && p2.x >= 0 && p2.y < dim && p2.y >= 0 {
                                antiCount += 1;
                                antinodes1.push(p2);
                            }
                        }
                    }
                    if antenna1.x - antenna2.x > 0 && antenna1.y - antenna2.y > 0 {
                        for i in 1..50 {
                            let p1 = Point{x:antenna1.x + distX*i, y: antenna1.y + distY*i};
                            if p1.x < dim && p1.x >= 0 && p1.y < dim && p1.y >= 0 {
                                antiCount += 1;
                                antinodes1.push(p1);
                            }
                            let p2 = Point{x:antenna2.x - distX*i, y: antenna2.y - distY*i};
                            if p2.x < dim && p2.x >= 0 && p2.y < dim && p2.y >= 0 {
                                antiCount += 1;
                                antinodes1.push(p2);
                            }
                        }
                    }
                    antinodes1.push(*antenna2);
                    antinodes1.push(*antenna1)
                }
            }
            seen.push(*antenna1)
        }
        for antinode in antinodes1 {
            if !antinodes.contains(&antinode) {
                antinodes.push(antinode);
            }
        }
    }
    println!("{:?}", antinodes.len());
}