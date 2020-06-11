use std::cmp;

fn main() {
    println!("Hello, world!");

    // let test = vec![
    //     vec![1,3,1],
    //     vec![1,5,1],
    //     vec![4,2,1]
    // ];

    let test = vec![
        vec![1,2,5],
        vec![3,2,1]
    ];

    let ans = Solution::min_path_sum(test);
    println!("{}", ans);
}

struct Solution {}

impl Solution {  
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let h = grid.len();
        let w = grid[0].len();

        let mut path_diction:Vec<Vec<i32>> = vec![vec![0; w]; h];
      
        for i in 0..h {
            for j in 0..w {
                if i == 0 && j == 0 {
                    path_diction[i][j] = grid[0][0];
                } else {
                    if i == 0 && j > 0 {
                        path_diction[0][j] = path_diction[0][j-1] + grid[0][j];
                    }
                    
                    if j == 0 && i > 0 {
                        path_diction[i][0] = path_diction[i-1][0] + grid[i][0];
                    }
                    
                    if i > 0 && j > 0 {
                        path_diction[i][j] = cmp::min(path_diction[i-1][j], path_diction[i][j-1]) + grid[i][j];
                    }
                }
            }
        }

        path_diction[h-1][w-1]
    }
}