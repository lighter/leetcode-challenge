// http://www.lowlevelmanager.com/2020/05/toy-interiew-problems-in-rust.html?m=0

fn main() {
    println!("Hello, world!");
}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn is_valid(root: &Option<Rc<RefCell<TreeNode>>>, arr: &Vec<i32>, start: i32) -> bool {
        if root.is_none() {
            return false;
        }
        if start >= arr.len() as i32 {
            return false;
        }
        if root.as_ref().unwrap().borrow().val != arr[start as usize] {
            return false;
        }
        if root.as_ref().unwrap().borrow().left.is_none() && root.as_ref().unwrap().borrow().right.is_none() {
            return start + 1 == arr.len() as i32;
        }

        return Self::is_valid(&(root.as_ref().unwrap().borrow().left), &arr, start + 1) || Self::is_valid(&(root.as_ref().unwrap().borrow().right), &arr, start + 1);
    }

    pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
        Self::is_valid(&root, &arr, 0)
    }
}

#[test]
pub fn test_singleton() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    let expected = vec![8];
    assert!(Solution::is_valid_sequence(tree, expected))
}

#[test]
pub fn test_small_tree() {
    // grungy hard coding of tree
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let tree = Some(Rc::new(RefCell::new(TreeNode{val:1, left, right})));
    
    let path = vec![1,2];
    let expected = true;
    assert!(Solution::is_valid_sequence(tree.clone(), path) == expected);
    
    let path = vec![1,3];
    let expected = true;
    assert!(Solution::is_valid_sequence(tree.clone(), path) == expected);
    
    let path = vec![1,2,3];
    let expected = false;
    assert!(Solution::is_valid_sequence(tree.clone(), path) == expected);
    
    let path = vec![1];
    let expected = false;
    assert!(Solution::is_valid_sequence(tree.clone(), path) == expected);
}