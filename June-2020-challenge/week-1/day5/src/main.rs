extern crate rand;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let w = vec![1,2,3];
    let obj = Solution::new(w);

    println!("{:?}", obj.w_data);
    println!("{:?}", obj.a_data);
    println!("{}", obj.total);

    let _ret_1: i32 = obj.pick_index();

    println!("{}", obj.total);

}

struct Solution {
    w_data: Vec<i32>,
    a_data: Vec<i32>,
    total: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let a_d:Vec<i32> = w.iter().scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        }).collect();

        let total = w.iter().sum();

        let s = Solution {
            w_data: w,
            a_data: a_d,
            total: total
        };

        s
    }
    
    fn pick_index(&self) -> i32 {
        let target = rand::thread_rng().gen_range(0, self.total);
        let mut l = 0;
        let mut r = self.a_data.len() as i32;

        while l < r {
            let mid = (l + r) / 2;
            if self.a_data[mid as usize] > target {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        l
    }
}