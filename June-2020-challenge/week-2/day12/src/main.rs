use std::collections::HashMap;
use rand::Rng;

fn main() {
    // println!("Hello, world!");
    let mut obj = RandomizedSet::new();
    obj.insert(0);
    println!("{:?}", obj.rd_set);
    println!("{:?}", obj.values);
    obj.insert(1);
    println!("{:?}", obj.rd_set);
    println!("{:?}", obj.values);
    obj.remove(0);
    println!("{:?}", obj.rd_set);
    println!("{:?}", obj.values);
    obj.insert(2);
    println!("{:?}", obj.rd_set);
    println!("{:?}", obj.values);
    obj.remove(1);
    println!("{:?}", obj.values);
    println!("{:?}", obj.rd_set);
    println!("{}", obj.get_random());
}

struct RandomizedSet {
    rd_set: HashMap<i32, usize>,
    values: Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        let rd_set = HashMap::new();
        RandomizedSet {rd_set: rd_set, values: vec![]}
    }
    
    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.rd_set.contains_key(&val) {
            return false;
        }

        let rd_len = self.rd_set.len();
        if self.values.len() > rd_len {
            self.values[rd_len] = val;
        } else {
            self.values.push(val);
        }
        self.rd_set.insert(val, rd_len);
        true
    }
    
    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if !self.rd_set.contains_key(&val) {
            return false;
        } 
        let v_index = *self.rd_set.get(&val).expect("must ok");
        if v_index != self.rd_set.len() - 1 {
            self.values[v_index] = self.values[self.rd_set.len() - 1];
            self.rd_set.insert(self.values[v_index], v_index);
        }

        self.rd_set.remove(&val);
        true
    }
    
    /** Get a random element from the set. */
    fn get_random(&self) -> i32 { 
        let mut rng = rand::thread_rng();
        let mut i: usize = rng.gen();
        i = i % self.rd_set.len();
        return self.values[i];
    }
}
