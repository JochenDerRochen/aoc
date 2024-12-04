fn main() {
    let input = include_str!("input.txt");
    let mut result = 0;
    let mut result2 = 0;
    let dim = 140;
    let mut grid = vec![vec!['.';0]; dim];
    for (lineid, line) in input.lines().enumerate(){

        for char in line.chars() {
            grid[lineid].push(char);
        }
        let mut crossword: String = line.clone().chars().collect();
        let horizontal:Vec<_> = crossword.match_indices("XMAS").map(|(i,_)| i).collect();
        let bwHorizontal:Vec<_> = crossword.match_indices("SAMX").map(|(i,_)|i).collect();
        result += horizontal.len() + bwHorizontal.len();
    }
    for (i, row) in grid.iter().enumerate() {
        for (j, column) in grid.iter().enumerate() {
            if grid[i][j] == 'X' || grid[i][j] == 'S' {
                let mut wordDiagR = String::from("");
                let mut wordVert = String::from("");
                let mut wordDiagL = String::from("");
                wordDiagL.push(grid[i][j]);
                wordDiagR.push(grid[i][j]);
                wordVert.push(grid[i][j]);
                for offset in 1..4 {
                    if i+offset < dim {
                        wordVert.push(grid[i+offset][j]);
                    }
                    if i+offset < dim && j+offset < dim {
                        wordDiagR.push(grid[i+offset][j+offset]);
                    }
                    if offset <= j && i+offset < dim {
                        wordDiagL.push(grid[i+offset][j-offset]);
                    }
                }
                if wordVert == "XMAS" || wordVert == "SAMX" {
                    result += 1;   
                }
                
                if wordDiagL == "XMAS" || wordDiagL == "SAMX" {
                    result += 1;   
                }

                if wordDiagR == "XMAS" || wordDiagR == "SAMX" {
                    result += 1;   
                }

            }
            if i > 0 && j > 0 && i < dim-1 && j < dim-1 {
                if grid[i][j] == 'A' {
                    let list = vec![grid[i-1][j-1], grid[i-1][j+1], grid[i+1][j-1], grid[i+1][j+1]];
                    let mut scount = 0;
                    let mut mcount = 0;
                    for i in &list {
                        if *i == 'M' {
                            mcount += 1;
                        }
                        if *i == 'S' { 
                            scount += 1;
                        }
                    }
                    if mcount == 2 && scount == 2 && list[0] != list[3] {
                        result2 += 1;
                    }
                }
            }
        }
    }
    println!("{}", result);
    println!("{}", result2);

}
