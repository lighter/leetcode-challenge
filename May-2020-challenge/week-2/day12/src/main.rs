fn main() {
    // println!("Hello, world!");
    let input = vec![1,1,2,3,3,4,4,8,8];
    let result = Solution::single_non_duplicate(input);
    
    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
      
        for (index, num) in nums.iter().enumerate() {
          if index != 0 {
            result = result ^ num;
          }
        }
        
        result
    }
}