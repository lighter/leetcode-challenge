fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result_vec = vec![];

        let mut left = 1;
        for num in nums.iter() {
            result_vec.push(left);
            left = left * num;
        }

        let mut right = 1;
        for i in (0..=nums.len()-2).rev() {
            right = right * nums[i+1];
            result_vec[i] = result_vec[i] * right;
        }

        result_vec
    } 
}

#[test] 
fn product_except_self_should_work() {
    let ans = vec![24, 12, 8, 6];
    let input = vec![1, 2, 3, 4];
    let output = Solution::product_except_self(input);

    assert_eq!(ans, output);
}