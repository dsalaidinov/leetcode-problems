Link: https://leetcode.com/problems/balanced-binary-tree/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
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
use core::cmp::max;
use libc::abs;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_height = Solution::height(node.borrow().left.clone());
            let right_height = Solution::height(node.borrow().right.clone());

            return max(left_height, right_height) + 1;
        } else {
            0
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let left_height = Solution::height(node.borrow().left.clone());
            let right_height = Solution::height(node.borrow().right.clone());

            if (left_height - right_height).abs() > 1 {
                return false;
            }
            return Solution::is_balanced(node.borrow().left.clone()) && Solution::is_balanced(node.borrow().right.clone());
        } else {
            true
        }
    }


}