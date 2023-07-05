fn main() {
    // println!("Hello, world!");
    let input = vec![];
    let result = Solution::h_index(input);
    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len() as i32;
        let mut left = 0;
        let mut right = len - 1;

        while left <= right {
            let mid = (right - left) / 2 + left;
            
            if citations[mid as usize] == len - mid {
                return len - mid;
            } else if citations[mid as usize] < len - mid{
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return len - left;
    }
}