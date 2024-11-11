Link: https://leetcode.com/problems/delete-leaves-with-a-given-value/

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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();

            let left = Solution::remove_leaf_nodes(node.left.take(), target);
            let right = Solution::remove_leaf_nodes(node.right.take(), target);

            node.left = left;
            node.right = right;

            if node.left.is_none() && node.right.is_none() && node.val == target {
                return None;
            }
        }
        root
    }
}