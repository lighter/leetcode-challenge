fn main() {
    
}

struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |a, b| { a ^ b })
    }
}

#[test]
fn single_number_should_work() {
    let nums = vec![1,2,4,5,1,2,4];

    let ans = Solution::single_number(nums);
    assert_eq!(ans, 5);
}