fn main() {
    // println!("Hello, world!");
    let test = String::from(")(");
    let result = Solution::check_valid_string(test);

    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut max_op = 0;
        let mut min_op = 0;
        
        for char in s.chars() {
            if char == '(' {
                max_op = max_op + 1;
            } else {
                max_op = max_op - 1;
            }

            if char != ')' {
                min_op = min_op + 1;
            } else {
                min_op = min_op - 1;
            }

            if (min_op < 0) {
                return false;
            }
            
            if (max_op < 0) {
                max_op = 0;
            }
        }

        return max_op == 0;
    }
}