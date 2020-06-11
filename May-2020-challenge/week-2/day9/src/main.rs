fn main() {
    // println!("Hello, world!");
    let input = 2147483647;
    let result = Solution::is_perfect_square(input);
    println!("{}", result);
}

struct Solution {} 

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut left:i64 = 1;
        let mut right:i64 = num as i64;

        while left <= right {
            let mid:i64 = (left + (right - left) / 2) as i64;
            let mid_square:i64 = mid * mid;

            if mid_square == num as i64{
                return true;
            } else if mid_square > num as i64{
                right = mid - 1 as i64;
            } else {
                left = mid + 1 as i64;
            }
        }

        false
    }
}