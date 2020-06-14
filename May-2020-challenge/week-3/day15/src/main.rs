use std::cmp;

fn main() {
    // println!("Hello, world!");
    let input = vec![5,-3,5];
    let result = Solution::max_subarray_sum_circular(input);
    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
        if a.len() == 0 {
            return 0;
        }

        let mut max_here = a[0];
        let mut max_total = a[0];
        let mut min_here = a[0];
        let mut min_total = a[0];
        let mut sum = a[0];

        for &i in a.iter().skip(1) {
            sum = sum + i;

            if max_here + i > i {
                max_here = max_here + i;
            } else {
                max_here = i;
            }

            max_total = cmp::max(max_total, max_here);

            if min_here + i < i {
                min_here = min_here + i;
            } else {
                min_here = i;
            }

            min_total = cmp::min(min_total, min_here);
        }

        if sum == min_total {
            return max_total;
        }

        return cmp::max(sum - min_total, max_total);
    }
}