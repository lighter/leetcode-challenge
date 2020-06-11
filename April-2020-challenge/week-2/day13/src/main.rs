use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut distance_map = HashMap::new();
        let mut ans = 0;
        let mut sum = 0;

        for (index, &num) in nums.iter().enumerate() {
            if num == 1 {
                sum = sum + 1;
            }

            if num == 0 {
                sum = sum - 1;
            }
        
            if sum == 0 {
                ans = index as i32 + 1;
            }
            else if distance_map.contains_key(&sum) {
                let distance = index as i32 - distance_map.get(&sum).unwrap();
                if distance > ans {
                    ans = distance;
                }
            } else {
                distance_map.insert(sum, index as i32);
            }
        }        

        ans as i32
    }
}

#[test]
fn find_max_length_should_work() {
    let test:Vec<i32> = vec![1, 0];
    let result = Solution::find_max_length(test);
    
    assert_eq!(2, result);
}