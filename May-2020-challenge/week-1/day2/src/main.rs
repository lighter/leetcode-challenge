fn main() {
    // println!("Hello, world!");
    let j = String::from ("aA");
    let s = String::from("aAAbbbb");

    let result = Solution::num_jewels_in_stones(j, s);
    println!("{}", result);
}


struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut answer = 0;

        for j_char in j.chars() {
            for s_cahr in s.chars() {
                if j_char == s_cahr {
                    answer = answer + 1;
                }
            }
        }  

        answer
    }
}