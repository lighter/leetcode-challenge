fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let ms = s.clone();
        let mut chars: Vec<char> = ms.chars().collect();
        let chars_length = chars.len();
        let mut total_amount = 0;
        
        for command in shift.iter() {
            let direction = command[0];
            let mut amount = command[1] % chars_length as i32;

            if direction == 1 {
                amount = chars_length as i32 - amount;
            }

            total_amount = total_amount + amount;
        }
        
        total_amount = total_amount % (chars_length as i32);
        let chars_copy = chars.clone();

        for i in 0..chars_length {
            chars[i] = chars_copy[(i + total_amount as usize) % chars_length];
        }

        chars.into_iter().collect()
    }
}

#[test]
fn string_shift_should_work() {
    let s = String::from("abc");
    let shift = vec![vec![0,1], vec![1,2]];
    let result = Solution::string_shift(s, shift);
    assert_eq!(String::from("cab"), result);
}