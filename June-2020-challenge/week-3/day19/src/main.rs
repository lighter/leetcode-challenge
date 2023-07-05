use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    let input: String = String::from("banana");
    let result = Solution::longest_dup_substring(input);

    println!("ans: {}", result);
}

struct Solution {}

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let mut left:i32 = 0;
        let mut right:i32 = s.len() as i32 - 1;
        let mut ans:String = String::from("");

        while left <= right {
            let mid = (right + left) / 2;
            println!("left:{} mid:{} right:{}", left, mid, right);
            let find = Self::rabin_karp(&s, mid as i32);
            if find.len() != 0 {
                left = mid + 1;
                ans = find;
            } else {
                right = mid - 1;
            }
        }
        
        ans
    }

    fn rabin_karp(s: &String, length:i32) -> String {
        if length == 0 {
            return String::from("");
        }

        const Q:i32 = 256;
        const B:i32 = 26;
        let mut hash_value = 0;
        let mut diction =  HashMap::new();

        for i in 0..length {
            let &char = &s.chars().nth(i as usize).unwrap();
            print!("{}{}", &char,length - i - 1);
            let char_code = Self::to_ascii(&char);
            // hash_value = hash_value + char_code * B.pow((length - i - 1) as u32);
            hash_value = (B * hash_value + char_code) % Q
        }

        print!(", value: {}\n",hash_value);
        diction.insert(hash_value, 0);

        let mut forget_base = 1;
        for _i in 0..length {
            forget_base = (forget_base * B) % Q;
        }

        println!("---{},{}---",s.len(), length);

        let loop_max = s.len() as i32 - length + 1;
        for i in 1..loop_max {
            let head_char = &s.chars().nth((i-1) as usize).unwrap();
            let head_char_code = Self::to_ascii(&head_char);

            let next_char = &s.chars().nth((i + length - 1) as usize).unwrap();
            let next_char_code = Self::to_ascii(&next_char);
            
            // hash_value = hash_value - (head_char_code * B.pow((length - 1) as u32));
            // hash_value = hash_value * B + (next_char_code * B.pow(0));

            hash_value = (B * (hash_value - head_char_code * forget_base) + next_char_code) % Q;

            println!("i: {}, length:{}, head: {}{}, next: {}{}, hash_value: {}", i, length, head_char, head_char_code, next_char, next_char_code, hash_value);

            for (key, value) in diction.iter() {
                if *key == hash_value {
                    println!("same value: {}", hash_value);
                    let ss:usize = i as usize;
                    let e:usize = (i+length) as usize;
                    let sk:usize = *value as usize;
                    let ek:usize = (*value+length) as usize; 
                    if &s.as_str()[ss..e] == &s.as_str()[sk..ek] {
                        let result = &s.as_str()[ss..e];
                        return String::from(result);
                    }
                }
            }

            diction.insert(hash_value, i);
        }

        String::from("")
    }

    fn to_ascii(c: &char) -> i32 {
        const RADIX: u32 = 10;

        return (*c as u32 - RADIX) as i32;
    }
}