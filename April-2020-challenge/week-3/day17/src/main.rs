fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn dfs(grid: &mut Vec<Vec<char>>, x:usize, y:usize, w:usize, h:usize) {
        if x >= w || y >= h || grid[x][y] == '0' {
            return;
        }
        grid[x][y] = '0';

        Self::dfs(grid, x + 1, y, w, h);
        if x > 0 {
            Self::dfs(grid, x - 1, y, w, h);
        }
        Self::dfs(grid, x, y + 1, w, h);

        if y > 0 {
            Self::dfs(grid, x, y - 1, w, h);
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;

        if grid.len() == 0 {
            return ans;
        }

        let mut clone_grid = grid.clone();

        let w = grid.len();
        let h = grid[0].len();

        for i in 0..w {
            for j in 0..h {
                if clone_grid[i][j] == '1' {
                    ans = ans + 1;
                }
                Self::dfs(&mut clone_grid, i, j, w, h);
            }
        }

        ans
    }
}