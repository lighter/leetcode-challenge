use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    let input = String::from("loveleetcode");
    let result = Solution::first_uniq_char(input);

    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut ans:i32 = -1;
        let mut char_map = HashMap::new();

        for c in s.chars() {
            match char_map.get_mut(&c) {
                Some(x) => {
                    *x = *x + 1;
                },
                None => {
                    char_map.insert(c, 1 as i32);
                }
            }
        }

        for (index, c) in s.chars().enumerate() {
            if char_map.get(&c).unwrap() == &(1 as i32) {
                ans = index as i32;
                break;
            }
        }

        ans
    }
}