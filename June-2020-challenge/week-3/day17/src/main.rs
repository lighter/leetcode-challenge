fn main() {
    // println!("Hello, world!");
    // let mut input = vec![
    //     vec!['X', 'O', 'X', 'O', 'X', 'O'],
    //     vec!['O', 'X', 'O', 'X', 'O', 'X'],
    //     vec!['X', 'O', 'X', 'O', 'X', 'O'],
    //     vec!['O', 'X', 'O', 'X', 'O', 'X'],
    // ];

    // let mut input = vec![
    //     vec!['X', 'X', 'X', 'X'],
    //     vec!['X', 'O', 'O', 'X'],
    //     vec!['X', 'X', 'O', 'X'],
    //     vec!['X', 'O', 'X', 'X'],
    // ];

    // let mut input = vec![vec!['O']];

    // let mut input = vec![
    //     vec!['X', 'X', 'X', 'X', 'X'],
    //     vec!['X', 'X', 'O', 'O', 'X'],
    //     vec!['X', 'X', 'X', 'O', 'X'],
    //     vec!['O', 'O', 'X', 'O', 'X'],
    //     vec!['X', 'O', 'O', 'X', 'X'],
    // ];

    let mut input = vec![
        vec!['X','O','O','X','X','X','O','X','O','O'],
        vec!['X','O','X','X','X','X','X','X','X','X'],
        vec!['X','X','X','X','O','X','X','X','X','X'],
        vec!['X','O','X','X','X','O','X','X','X','O'],
        vec!['O','X','X','X','O','X','O','X','O','X'],
        vec!['X','X','O','X','X','O','O','X','X','X'],
        vec!['O','X','X','O','O','X','O','X','X','O'],
        vec!['O','X','X','X','X','X','O','X','X','X'],
        vec!['X','O','O','X','X','O','X','X','O','O'],
        vec!['X','X','X','O','O','X','O','X','X','O']
    ];
    Solution::solve(&mut input);

    println!("{:?}", input);
}

struct Solution {}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let y = board.len();
        if y == 0 { return; }
        let x = board[0].len();

        for i in 0..y {
            Self::dfs(board, 0, i);
            Self::dfs(board, x - 1, i);
        }

        for j in 0..x {
            Self::dfs(board, j, 0);
            Self::dfs(board, j, y - 1);  
        }

        for row in board {
            for coloumn in row {
                if *coloumn == 'O' {
                    *coloumn = 'X';
                }
                if *coloumn == 'G' {
                    *coloumn = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, x:usize, y:usize) {
        if x < 0 as usize || x >= board[0].len() || y < 0 as usize || y >= board.len() || board[y][x] != 'O' {
            return;
        }
        // println!("x: {}, y: {}, c: {}", x, y, board[y][x]);

        board[y][x] = 'G';

        if x > 0 && y > 0 {
            Self::dfs(board, x - 1, y);
            Self::dfs(board, x, y - 1);
        }

        Self::dfs(board, x + 1, y);
        Self::dfs(board, x, y + 1);
    }
}