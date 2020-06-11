fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total = 0;
        for (index, &price) in prices.iter().enumerate() {
            if (index + 1) < prices.len() {
                if price < prices[index + 1] {
                    total = total + (prices[index + 1] - price);
                
                }
            }
        }

        total
    }
}

#[test]
fn max_profit_should_work() {
    let price = vec![7,1,5,3,6,4];
    let ans = 7;
    let result = Solution::max_profit(price);
    assert_eq!(result, ans);
}