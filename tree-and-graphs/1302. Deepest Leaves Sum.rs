Link: https://leetcode.com/problems/deepest-leaves-sum/description

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
use std::collections::VecDeque;

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut deepest_sum = 0;

        if let Some(root_node) = root {
            queue.push_back(root_node);
        }

        while !queue.is_empty() {
            let level_size = queue.len();
            deepest_sum = 0; 

            for i in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node_borrowed = node.borrow();
                    deepest_sum += node_borrowed.val;

                    if let Some(left) = &node_borrowed.left {
                        queue.push_back(left.clone());
                    }

                    if let Some(right) = &node_borrowed.right {
                        queue.push_back(right.clone());
                    }
                }
            }
        }

        deepest_sum
  
    }
}