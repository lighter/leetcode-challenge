use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    // println!("Hello, world!");
    let nums = vec![8, 5];
    Solution::bst_from_preorder(nums);
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
    pub fn add_to_binary_tree(num: i32, node: &Option<Rc<RefCell<TreeNode>>>) {
        match node.clone() {
            Some(node) => {
                let val = node.borrow().val;

                if num > val {
                    if node.borrow().right.is_none() {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new(
                            TreeNode::new(num)
                        )));
                    } else {
                        Self::add_to_binary_tree(num, &node.borrow().right);
                    }
                } else {
                    if node.borrow().left.is_none() {
                        node.borrow_mut().left = Some(Rc::new(RefCell::new(
                            TreeNode::new(num)
                        )));
                    } else {
                        Self::add_to_binary_tree(num, &node.borrow().left);
                    }
                }
            },  
            None => println!("Cannot divide by 0"),
        }
    }

    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_node:Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(
            TreeNode::new(preorder[0])
        )));

        for (index, &p) in preorder.iter().enumerate() {
            if index > 0 {
                Self::add_to_binary_tree(p, &root_node);        
            }
        }

        // println!("{:?}", root_node);

        root_node
    }
}