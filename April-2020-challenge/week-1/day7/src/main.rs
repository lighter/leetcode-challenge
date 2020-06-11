fn main() {
    let nums = vec![1,3,2,3,5,0];

    let result = Solution::count_elements(nums);
    println!("{}", result);

}

struct Solution {}

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let max_length: usize = 10000;
        let min_length: usize = 1;
        let max_num: i32 = 1000;
        let min_num: i32 = 0;

        if arr.len() > max_length || arr.len() <= min_length {
            count
        } else {
            for n in arr.iter() {
                if n <= &max_num && &min_num <= n {
                    let index = arr.iter().find(|&&r| r == n+1);
                    if index.is_some() {
                        count = count + 1;
                    }
                }
            }

            count
        }
    }
}

#[test]
fn count_elements_should_work() {
    let nums = vec![1,3,2,3,5,0];
    let _ans = 3;
    let result = Solution::count_elements(nums);

    println!("{}", result);
    // assert_eq!(3, result);
}