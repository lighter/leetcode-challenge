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

struct Solution {}

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(v) = node {
                let c = v.borrow();
                let c_v = c.val;
                if c_v == val {
                    return Some(Rc::clone(v)); 
                } else {
                    let next = if val > c_v {&c.right} else {&c.left};
                    return helper(&next, val);
                }
            }
            
            None
        }
        
        return helper(&root, val);
    }
}