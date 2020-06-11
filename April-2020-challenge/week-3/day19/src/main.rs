fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        
        if nums.len() == 0 {
            return - 1;
        }

        while left <= right {
            let mid = (left + right) / 2;

            if target == nums[mid] {
                return mid as i32;
            } else {
                if nums[mid] >= nums[left] {
                    if target >= nums[left] && target < nums[mid] {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                } else {
                    if target >= nums[mid] && target < nums[left] {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
            }
        }

        return -1;
    }
}

#[test] 
fn search_should_work() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    let ans = 4;
    let result = Solution::search(nums, target);

    assert_eq!(ans, result);
}