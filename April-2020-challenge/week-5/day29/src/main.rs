use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
use std::i32;

fn main() {
    // println!("Hello, world!");
    let tn = TreeNode::new(-3);
    Solution::max_path_sum(Some(Rc::new(RefCell::new(tn))));
}

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
    pub fn max_path_sum_with_root(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root_value = root.clone().unwrap().borrow().val;
        let left = root.clone().unwrap().borrow().left.clone();
        let left_value = Self::max_path_sum_with_root(left) + root_value;

        let right = root.clone().unwrap().borrow().right.clone();
        let right_value = Self::max_path_sum_with_root(right) + root_value;

        let none_select = 0;
        cmp::max(cmp::max(left_value, right_value), none_select)
    }

    pub fn max_path_root(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root_value = root.clone().unwrap().borrow().val;
        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();
        
        let middle_value = Self::max_path_sum_with_root(left) + root_value + Self::max_path_sum_with_root(right);

        middle_value
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return i32::MIN;
        }

        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();
        
        let middle_value = Self::max_path_root(root.clone());
        let right_value = Self::max_path_sum(right);
        let left_value = Self::max_path_sum(left);

        cmp::max(cmp::max(middle_value, right_value), left_value)
    }
}