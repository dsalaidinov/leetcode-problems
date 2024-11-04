Link: https://leetcode.com/problems/insert-into-a-binary-search-tree
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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut borrowed_node = node.borrow_mut();

            if val < borrowed_node.val {
                borrowed_node.left = Solution::insert_into_bst(borrowed_node.left.clone(), val);
            } else {
                borrowed_node.right = Solution::insert_into_bst(borrowed_node.right.clone(), val);
            }

            return Some(node.clone());
        } else {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }
}