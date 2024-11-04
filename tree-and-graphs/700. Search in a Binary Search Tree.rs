Link: https://leetcode.com/problems/search-in-a-binary-search-tree

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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let node_borrowed = node.borrow();

            if node_borrowed.val == val {
                return Some(node.clone())
            } else if val < node_borrowed.val {
                return Solution::search_bst(node_borrowed.left.clone(), val);
            } else {    
                return Solution::search_bst(node_borrowed.right.clone(), val);
            }
        }
        None
    }
}