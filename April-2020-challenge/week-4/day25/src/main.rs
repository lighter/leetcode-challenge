fn main() {
    // println!("Hello, world!");
    let input = vec![1];
    let result = Solution::can_jump(input);

    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach:i32 = 0;

        for i in 0..nums.len() {
            if i as i32 <= max_reach {
                let can_reach = i as i32 + nums[i];
                if can_reach > max_reach {
                    max_reach = can_reach;
                }
            }
        }

        nums.len() as i32 - 1 <= max_reach
    }
}