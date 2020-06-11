fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let new_s = Self::deal_with_string(s);
        let new_t = Self::deal_with_string(t);

        new_s == new_t
    }

    pub fn deal_with_string(str: String) -> String {
        let chars: Vec<char> = str.chars().collect();
        let mut new_chars = vec![];

        for index in 0..chars.len() {
            if chars[index] != '#' {
                new_chars.push(chars[index]);
            } else {
                if new_chars.len() > 0 {
                    new_chars.remove(new_chars.len() - 1);
                }
            }
        }      

        new_chars.iter().collect()
    }
}

#[test]
fn backspace_compare_should_work() {
    let s = String::from("ab#c");
    let t = String::from("ad#c");

    let result = Solution::backspace_compare(s, t);
    assert_eq!(result, true);
}