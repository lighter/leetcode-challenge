
fn main() {
    println!("{}", Solution::is_happy(19));
}

struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut ans = n;

        while ans >= 10 {
            ans = Self::square(ans);
        }

        ans == 1 || ans == 7
    }

    pub fn square(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            let d = n % 10;
            result += d.pow(2);
            n /= 10;
        }
        result
    }
}

#[test]
fn is_happy_should_work() {
    let num = 7;

    let ans = Solution::is_happy(num);
    assert_eq!(ans, true);
}