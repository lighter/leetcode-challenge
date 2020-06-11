use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Hello, world!");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

impl Solution {
    pub fn deep(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            Some(node) => {
                let left_max = Solution::deep(node.borrow().left.clone());
                let right_max = Solution::deep(node.borrow().right.clone());

                if (left_max > right_max) {
                    left_max + 1
                } else {
                    right_max + 1
                }
            },
            None => 0
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let middle = Solution::deep(root.borrow().left.clone()) + Solution::deep(root.borrow().right.clone());
                let left = Solution::deep(root.borrow().left.clone());
                let right = Solution::deep(root.borrow().right.clone());
                
                let mut max = middle;

                if left > max {
                    max = left;
                } 
                if right > max {
                    max = right;
                }

                max
            },
            None => 0
        }
    }
}