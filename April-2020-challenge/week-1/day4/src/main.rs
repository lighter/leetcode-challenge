fn main() {
    println!("Hello, world!");
}

struct Solution {} 

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut x = 0;
        let mut y = 0;
        let mut temp;
        while y < nums.len() {
            if nums[y] == 0 {
                y = y+1;
            } else {
                temp = nums[y];
                nums[y] = nums[x];
                nums[x] = temp;
                x = x+1;
                y = y+1;
            }
        }
    }
}

#[test]
fn move_zeroes_should_work() {
    let mut nums = vec![0,1,0,3,12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1,3,12,0,0]);
}