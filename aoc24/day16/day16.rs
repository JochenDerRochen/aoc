use std::fmt;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};


const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(PartialEq, Eq)]
struct State {
    x: i32,
    y: i32,
    score: usize,
    di: usize
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq,PartialEq, Hash, PartialOrd)]
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
    let mut map = Vec::<Vec<char>>::new();
    let mut start = Point{x:0, y:0};
    let mut end = Point{x:0, y:0};
    let mut heap = BinaryHeap::new();
    let width = 141;
    let height = 141;
    for (i, line) in lines.enumerate() {
        let mut row = Vec::<char>::new();
        for (j, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'E'{
                end = Point{x:j as i32, y:i as i32};
            }
            if c == 'S' {
                start = Point{x:j as i32, y:i as i32};
            }
        }
        map.push(row);
    }
    let startState = State{
        x: start.x,
        y: start.y,
        score: 0,
        di: 0,
    };
    heap.push(Reverse(startState));
    let mut seen = vec![usize::MAX; width*height*4];
    let mut min = usize::MAX;
    while let Some(Reverse(State {
        score,
        x,
        y,
        di: prev_di,
    })) = heap.pop() {
        if map[y as usize][x as usize] == 'E' {
            if score > min {
                break;
            }
            min = score;
        }
        for (di, (dx, dy)) in DIRS.iter().enumerate() {
            if (prev_di + 2) % DIRS.len() == di {
                // don't go back
                continue;
            }

            let nscore = if di == prev_di {
                // walk forwards
                score + 1
            } else {
                // turn and take one step
                score + 1001
            };

            let nx = x + dx;
            let ny = y + dy;

            let gi = ny as usize * width + nx as usize;
            let si = gi * DIRS.len() + di;
            let last_seen_score = seen[si];

            if map[ny as usize][nx as usize] != '#' && nscore <= last_seen_score {
                // save score for this place and direction
                seen[si] = nscore;
                heap.push(Reverse(State {
                    score: nscore,
                    x: nx,
                    y: ny,
                    di,
                }));
            }
        }
    }
    let mut total = 1; // include end position
    let mut places_to_sit = vec![false; width * height];
    let mut queue = VecDeque::new();
    queue.push_back((end, min));
    while let Some((node, score)) = queue.pop_front() {
        for di in 0..DIRS.len() {
            let next_score = seen[(node.y as usize * width + node.x as usize) * DIRS.len() + di];
            if next_score <= score {
                // walk back
                let nextx = node.x - DIRS[di].0;
                let nexty = node.y - DIRS[di].1;

                if !places_to_sit[nexty as usize * width + nextx as usize] {
                    places_to_sit[nexty as usize * width + nextx as usize] = true;
                    total += 1;
                    queue.push_back((Point{x: nextx, y: nexty}, next_score));
                }
            }
        }
    }
    println!("{}", total);
    println!("{}", min);
    
}