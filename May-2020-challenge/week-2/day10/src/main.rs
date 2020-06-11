use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    let n = 1;
    let trust = vec![];

    let result = Solution::find_judge(n, trust);

    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return n;
        }

        let mut judge_map = HashMap::new();

        for t in trust {
            match judge_map.get_mut(&t[1]) {
                Some(x) => {
                    *x = *x + 1;
                },
                None => {
                    judge_map.insert(t[1], 1);
                }
            }
            match judge_map.get_mut(&t[0]) {
                Some(x) => {
                    *x = *x - 1;
                },
                None => {
                    judge_map.insert(t[0], -1);
                }
            }
        }

        for i in 1..=n {
            if let Some(x) = judge_map.get(&i) {
                if *x == n - 1 {
                    return i;
                }
            }
        }

        -1
    }
}