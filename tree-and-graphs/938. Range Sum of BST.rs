Link: https://leetcode.com/problems/range-sum-of-bst/

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(node) = root {
            let mut borrowed_node = node.borrow_mut();
            let mut sum = 0;
            
            if borrowed_node.val >= low && borrowed_node.val <= high {
                sum += borrowed_node.val;
            } 

            sum += Solution::range_sum_bst(borrowed_node.left.clone(), low, high);
            sum += Solution::range_sum_bst(borrowed_node.right.clone(), low, high);

            return sum as i32
        }

        0
    }
}