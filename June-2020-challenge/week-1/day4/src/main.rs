fn main() {
    // println!("Hello, world!");
    let mut input:Vec<char> = vec!['a'];
    Solution::reverse_string(&mut input);
    println!("{:?}", input);
}

struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() > 0 {
            let s_len = s.len() - 1;
            for i in 0..s_len {
                if i <= s_len / 2 {
                    s.swap(i, s_len - i);
                }
            }
        }
    }
}