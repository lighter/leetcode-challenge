fn main() {
    // println!("Hello, world!");
    let input = vec![
        vec![10,20],
        vec![30,200],
        vec![400,50],
        vec![30,20]
    ];

    let result = Solution::two_city_sched_cost(input);
    println!("{}", result);
}

struct Solution {}

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let len = costs.len();
        let mut dis:i32 = 0;
        let mut cm = costs.clone();

        for cost in costs {
            dis = dis + cost[0];
        }

        cm.sort_by(|a, b| (b[0]-b[1]).cmp(&(a[0]-a[1])));

        for (index, c) in cm.iter().enumerate() {
            if index < len / 2 {
                dis = dis - c[0] + c[1];
            }
        }

        dis
    }
}