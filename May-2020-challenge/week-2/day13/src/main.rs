fn main() {
    // println!("Hello, world!");
    let input = String::from("112");
    let k:i32 = 1;
    let result = Solution::remove_kdigits(input, k);

    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        if num.len() as i32 == k {
            return String::from("0");
        }

        let mut char_vec: Vec<char> = num.chars().collect();

        for _j in 0..k {
            let mut i = 0;
            while i < char_vec.len() as i32 - 1 && char_vec[i as usize] <= char_vec[(i+1) as usize] {
                i = i + 1;
            }

            char_vec.remove(i as usize);
        }

        while char_vec.len() > 1usize && char_vec[0] == '0' {
            char_vec.remove(0);
        }

        if char_vec.len() == 0 {
            return String::from("0");
        }

        char_vec.iter().collect()
    }
}
