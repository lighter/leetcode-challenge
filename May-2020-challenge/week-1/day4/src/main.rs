fn main() {
    // println!("Hello, world!");
    let input:i32 = 7;
    let ans = Solution::find_complement(input);

    println!("{}", ans);
}

struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let base:i32 = 2;
        let binary_num = format!("{:b}", num);
        let mut ans = 0;
        
        for (index, ch) in binary_num.chars().rev().enumerate() {
            if ch == '0' {
                ans = ans + base.pow(index as u32);
            }
        }
      
        ans
    }
}