use leetcode_prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // println!("Hello, world!");
    let tree = btree![1,2,3,null,4,null,5];
    let result = Solution::is_cousins(tree, 5, 4);
    println!("{}", result);
}


// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

struct Solution {}

impl Solution {
    fn depth(root: &Option<Rc<RefCell<TreeNode>>>, target:i32, d:i32) -> i32 {

        if root.is_none() {
            return -1;
        }

        if root.as_ref().unwrap().borrow().val == target {
            return d;
        } 

        let left_depth = Self::depth(&root.as_ref().unwrap().borrow().left, target, d+1);
        let right_depth = Self::depth(&root.as_ref().unwrap().borrow().right, target, d+1);

        if left_depth != -1 {
            return left_depth;
        }

        if right_depth != -1 {
            return right_depth;
        }

        -1
    }

    fn parent(root: &Option<Rc<RefCell<TreeNode>>>, target: i32, parent: i32) -> i32 {
        if root.is_none() {
            return -1;
        }

        if  root.as_ref().unwrap().borrow().val == target {
            return parent;
        }

        let left_parent = Self::parent(&root.as_ref().unwrap().borrow().left, target, root.as_ref().unwrap().borrow().val);
        let right_parent = Self::parent(&root.as_ref().unwrap().borrow().right, target, root.as_ref().unwrap().borrow().val);

        if left_parent != -1 {
            return left_parent;
        }

        if right_parent != -1 {
            return right_parent;
        }

        -1
    }

    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let dx = Self::depth(&root, x, 0);
        let dy = Self::depth(&root, y, 0);
        let px = Self::parent(&root, x, 0);
        let py = Self::parent(&root, y, 0);

        if dx == dy && px != py {
            return true
        }

        false
    }
}