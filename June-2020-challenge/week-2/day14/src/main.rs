use std::iter::repeat;

fn main() {
    // println!("Hello, world!");
    let n = 3;
    let edges = vec![vec![0,1,100], vec![1,2,100], vec![0,2,500]];
    let src = 0;
    let dst = 2;
    let k = 1;

    let result = Solution::find_cheapest_price(n, edges, src, dst, k);
    println!("{}", result);
}
#[derive(Clone, Debug)]
struct Edge {
    to: i32,
    cost: i32
}

struct Solution {} 

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph: Vec<Vec<Edge>> = repeat(Vec::new()).take(n as usize).collect();
        for e in &flights {
            graph[e[0] as usize].push(Edge {
                to: e[1],
                cost: e[2],
            });
        }

        let mut vis = vec![false; n as usize];
        vis[src as usize] = true;
        let mut min_cost = i32::max_value();
        Solution::dfs(&graph, src, dst, 0, k, 0, &mut min_cost, &mut vis);
        if min_cost == i32::max_value() { -1 }
        else { min_cost }
    }
    fn dfs(graph: &Vec<Vec<Edge>>, src: i32, dst: i32, cur_step: i32, k: i32, cost: i32, min_cost: &mut i32, vis: &mut Vec<bool>) {
        if cur_step >= k + 2 || cost > *min_cost { return; }
        if src == dst { *min_cost = (*min_cost).min(cost); return; }

        for e in &graph[src as usize] {
            if !vis[e.to as usize] {
                vis[e.to as usize] = true;
                Solution::dfs(graph, e.to, dst, cur_step + 1, k, cost + e.cost, min_cost, vis);
                vis[e.to as usize] = false;
            }
        }
    }
}