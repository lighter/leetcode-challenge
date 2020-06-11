use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    let nums = vec![1,1,1];
    let target:i32 = 2;
    let result = Solution::subarray_sum(nums, target);

    println!("{}", result);
}


struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut sum_map = HashMap::new();

        let mut total_counter:i32 = 0;

        for j in 0..nums.len() {
            // println!("before: {:?}", sum_map);
            *sum_map.entry(sum).or_insert(0) += 1;
            sum = sum + nums[j];
            let target:i32 = sum - k;
            
            total_counter = total_counter + sum_map.get(&target).unwrap_or(&0);
            // println!("after: {}, {}, {:?}",total_counter, target, sum_map);
        }
        
        total_counter
    }

    // version 1
    // pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    //     let mut sum:Vec<i32> = vec![0; nums.len()+1];
    //     let mut count = 0;

    //     for j in 0..nums.len() {
    //         sum[j+1] = sum[j] + nums[j];

    //         let target:i32 = sum[j+1] - k;

    //         for i in 0..=j {
    //             if sum[i] == target {
    //                 count = count + 1;
    //             }
    //         }
    //     }

    //     count
    // }
}

#[test]
fn subarray_sum_should_work() {
    let nums = vec![1,2,3];
    let target:i32 = 3;
    let answer = 2;
    let result = Solution::subarray_sum(nums, target);
    assert_eq!(answer, result);
}