fn main() {
    // println!("Hello, world!");

    let m:i32 = 5;
    let n:i32 = 7;
    Solution::range_bitwise_and(m, n);
    Solution::range_bitwise_and_2(m, n);
}

struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {

        let mut res = 0;
        let mut difference = false;

        for i in (0..30).rev() {
            let dm = m & (1 << i);
            let dn = n & (1 << i);

            if dm != dn {
                difference = true;
            }

            if !difference {
                res = res + dm;
            }
        }

        res
    }

    pub fn range_bitwise_and_2(m: i32, n: i32) -> i32 {
        let mut count = 0;
        let mut cm = m;
        let mut cn = n;

        while cm != cn {
            cm = cm >> 1;
            cn = cn >> 1;
            count = count + 1;
        }

        cm << count
    }
}

#[test] 
fn range_bitwise_and_should_work() {
    let m:i32 = 5;
    let n:i32 = 7;
    let ans = 4;
    let result = Solution::range_bitwise_and(m, n);
    assert_eq!(ans, result);
}

#[test] 
fn range_bitwise_and_2_should_work() {
    let m:i32 = 5;
    let n:i32 = 7;
    let ans = 4;
    let result = Solution::range_bitwise_and_2(m, n);
    assert_eq!(ans, result);
}