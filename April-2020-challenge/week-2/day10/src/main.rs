fn main() {
    println!("Hello, world!");
}

struct MinStack {
    nums: Vec<i32>,
}

impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        let min_stack = MinStack { nums: Vec::new()};
        min_stack
    }
    
    fn push(&mut self, x: i32) {
        self.nums.push(x);
    }
    
    fn pop(&mut self) {
        self.nums.pop();
    }
    
    fn top(&self) -> i32 {
        self.nums[self.nums.len() - 1]
    }
    
    fn get_min(&self) -> i32 {
        *self.nums.iter().min().unwrap()
    }
}

#[test] 
fn min_stack_should_work() {
    let mut m = MinStack::new();
    MinStack::push(&mut m, 10);
    MinStack::push(&mut m, 20);
    MinStack::push(&mut m, 50);
    assert_eq!(vec![10, 20, 50], m.nums);

    MinStack::pop(&mut m);
    assert_eq!(vec![10, 20], m.nums);

    MinStack::push(&mut m, 5);
    assert_eq!(vec![10, 20, 5], m.nums);

    let num = MinStack::top(&mut m);
    assert_eq!(5, num);

    let min_num = MinStack::get_min(&mut m);
    assert_eq!(5, min_num);
}