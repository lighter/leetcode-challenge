use std::cmp;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }

        let mut answer = 0;
        let row_len = matrix[0].len()+1;
        let col_len = matrix.len()+1;

        let mut matrix_cache = vec![vec![-1; row_len]; col_len];

        for i in 0..col_len {
            for j in 0..row_len {
                if i == 0 || j == 0 {
                    matrix_cache[i][j] = 0;
                    continue;
                }

                if matrix[i-1][j-1] == '0' {
                    matrix_cache[i][j] = 0;
                } else {
                    matrix_cache[i][j] = cmp::min(
                        cmp::min(
                            matrix_cache[i-1][j],
                            matrix_cache[i][j-1]
                        ), 
                        matrix_cache[i-1][j-1]) + 1;
                }

                answer = cmp::max(answer, matrix_cache[i][j]);
            }
        }

        answer * answer
    }
}

#[test]
fn maximal_square_should_work() {
    let input = vec![vec!['1']];
    let result = Solution::maximal_square(input);

    assert_eq!(1, result);
}