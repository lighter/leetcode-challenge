fn main() {
    // println!("Hello, world!");

    let test:Vec<i32> = vec![4, 3, 1, 6, 2, 9];
    let ans = Solution::last_stone_weight(test);

    println!("{}", ans);
}

struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        if stones.len() < 2 {
            return stones[0];
        }

        let mut my_stones = stones.clone();

        loop {
            let c = |a: &i32,b: &i32|b.cmp(a);
            my_stones.sort_by(c);

            let y = my_stones[0];
            let x = my_stones[1];
            my_stones[0] = 0;
            my_stones[1] = 0;

            if x == 0 {
                return y;
            }

            if x != y {
                my_stones[0] = y - x;
            }
        }
    }
}