use std::fmt;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::cmp;


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct State {
    cost: i32,
    coord: Point
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(PartialEq,Eq, Ord, Hash, PartialOrd)]
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
    let mut obstacles = Vec::<Point>::new();
    let dim = 71;
    let mut maxObs = 1024;
    loop {
        maxObs += 1;
        let mut obstacles = Vec::<Point>::new();
    
    for line in input.lines() {
        let sep = line.find(",").unwrap();
        let xStr:String = line.chars().take(sep).collect();
        let yStr: String = line.chars().skip(sep+1).collect();
        let x = xStr.parse::<i32>().unwrap();
        let y = yStr.parse::<i32>().unwrap();
        obstacles.push(Point{x: x, y: y});
        if obstacles.len() == maxObs {
            break;
        }
    }
    let end = Point{x: dim-1, y: dim-1};
    let start =Point{x: 0, y:0};
    let mut search = BinaryHeap::new();
    search.push(State{cost: 0, coord: start});
    let mut costs = HashMap::<Point, i32>::new();
    costs.insert(start, i32::MAX);
    let mut current = State{cost:0, coord: start};
    while search.len() != 0 {
        current = search.pop().unwrap();

        if current.coord == end {
            //println!("{}", current.cost);
            break;
        }
        if current.cost > costs[&current.coord] {
            continue;
        }

        let cost = current.cost + 1;
        let p1 = State{cost: cost, coord: Point{x: current.coord.x + 1, y: current.coord.y}};
        if !costs.contains_key(&p1.coord) && p1.coord.x < dim {
            costs.insert(p1.coord, i32::MAX);
        }

        let p2 = State{cost: cost, coord: Point{x: current.coord.x - 1, y: current.coord.y}};
        if !costs.contains_key(&p2.coord) && p2.coord.x >= 0{
            costs.insert(p2.coord, i32::MAX);
        }
        let p3 = State{cost: cost, coord: Point{x: current.coord.x, y: current.coord.y-1}};
        if !costs.contains_key(&p3.coord) && p3.coord.y >= 0{
            costs.insert(p3.coord, i32::MAX);
        }
        let p4 = State{cost: cost, coord: Point{x: current.coord.x, y: current.coord.y+1}};
        if !costs.contains_key(&p4.coord) && p4.coord.y < dim {
            costs.insert(p4.coord, i32::MAX);
        }

        if p1.coord.x < dim && !(*costs.get(&p1.coord).unwrap() <= cost) && !obstacles.contains(&p1.coord) {
            search.push(p1.clone());
            *costs.get_mut(&p1.coord).unwrap() = cost; 
        }
        if p2.coord.x >= 0 && !(*costs.get(&p2.coord).unwrap() <= cost) && !obstacles.contains(&p2.coord) {
            search.push(p2.clone());
            *costs.get_mut(&p2.coord).unwrap() = cost; 
        }
        if p3.coord.y >= 0 && !(*costs.get(&p3.coord).unwrap() <= cost) && !obstacles.contains(&p3.coord) {
            search.push(p3.clone());
            *costs.get_mut(&p3.coord).unwrap() = cost; 
        }
        if p4.coord.y < dim  && !(*costs.get(&p4.coord).unwrap() <= cost) && !obstacles.contains(&p4.coord) {
            search.push(p4.clone());
            *costs.get_mut(&p4.coord).unwrap() = cost; 
        }
    }
    if current.coord != end {
        println!("{}", maxObs);
        break;
    }
    } 
}
