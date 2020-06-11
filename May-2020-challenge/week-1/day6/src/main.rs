use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    let input:Vec<i32> = vec![2,2,1,1,1,2,2];
    let result = Solution::majority_element(input);

    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut ans:i32 = -1;
        let mut max_count:i32 = 0;
        let limit = nums.len() as i32 / 2;
        let mut nums_map = HashMap::new();

        for num in nums {
            match nums_map.get_mut(&num) {
                Some(x) => {
                    *x = *x + 1;
                },
                None => {
                    nums_map.insert(num, 1);
                }
            }
        }

        for (num, count) in nums_map {
            if count > limit && count > max_count {
                ans = num;
                max_count = count;
            }
        }

        ans
    }
}