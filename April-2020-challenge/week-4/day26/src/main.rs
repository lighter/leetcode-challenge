fn main() {
    // println!("Hello, world!");

    let text1 = String::from("abcde");
    let text2 = String::from("ace");
    let result = Solution::longest_common_subsequence(text1, text2);

    println!("{}", result);
}

struct Solution {}

impl Solution {
    // ############### version 1 ###############

    // pub fn lcs(text1: &String, text2: &String, m: i32, n: i32, cache: &mut Vec<Vec<i32>>) -> i32{
    //     if m == 0 || n == 0 {
    //         return 0;
    //     }

    //     if cache[m as usize][n as usize] != -1 {
    //         return cache[m as usize][n as usize];
    //     }

    //     let result;

    //     if &text1.get((m-1) as usize..m as usize).unwrap() == &text2.get((n-1) as usize..n as usize).unwrap() {
    //         result = Self::lcs(&text1, &text2, m-1, n-1, cache) + 1;
    //     } else {
    //         let distance1 = Self::lcs(&text1, &text2, m, n - 1, cache);
    //         let distance2 = Self::lcs(&text1, &text2, m - 1, n, cache);

    //         if distance1 >= distance2 {
    //             result = distance1;
    //         } else {
    //             result = distance2;
    //         }
    //     }

    //     cache[m as usize][n as usize] = result;

    //     result
    // }

    // pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    //     let mut cache:Vec<Vec<i32>> = vec![vec![-1; text2.len() + 1]; text1.len() + 1];

    //     let len1:i32 = text1.len() as i32;
    //     let len2:i32 = text2.len() as i32;

    //     return Self::lcs(&text1, &text2, len1, len2, &mut cache);
    // }


    // ############### version 2 ###############

    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut cache:Vec<Vec<i32>> = vec![vec![-1; text2.len() + 1]; text1.len() + 1];

        let len1 = text1.len();
        let len2 = text2.len();

        for m in 0..=len1 {
            for n in 0..=len2 {
                if m == 0 || n == 0 {
                    cache[m][n] = 0;
                    continue;
                }

                let result;

                if text1.get((m as i32 - 1) as usize..m).unwrap() == text2.get((n as i32 - 1) as usize..n).unwrap(){
                    result = cache[(m as i32-1) as usize][(n as i32-1) as usize] + 1;
                } else {
                    let distance1 = cache[m][(n as i32 - 1) as usize];
                    let distance2 = cache[(m as i32 - 1) as usize][n];

                    if distance1 >= distance2 {
                        result = distance1;
                    } else {
                        result = distance2;
                    }
                }
                cache[m][n] = result;
            }
        }

        return cache[len1][len2];
    }
}