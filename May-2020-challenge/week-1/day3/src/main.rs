fn main() {
    // println!("Hello, world!");
    let ransom_note = String::from("aa");
    let magazine = String::from("aab");
    let result = Solution::can_construct(ransom_note, magazine);

    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count_ranson_note = vec![0; 26];
        let mut count_magazine = vec![0; 26];
        let a = 'a' as u32;

        for rn_char in ransom_note.chars() {
            let rn_char_num = rn_char as u32;
            let index = (rn_char_num - a) as usize;
            count_ranson_note[index] = count_ranson_note[index] + 1;
        }

        for mag_char in magazine.chars() {
            let mag_num = mag_char as u32;
            let index = (mag_num - a) as usize;
            count_magazine[index] = count_magazine[index] + 1;
        }

        for i in 0..26 {
            if count_ranson_note[i] > count_magazine[i] {
                return false;
            }
        }

        true
    }
}