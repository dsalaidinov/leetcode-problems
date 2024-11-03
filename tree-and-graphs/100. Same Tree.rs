Link: https://leetcode.com/problems/same-tree/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(node1), Some(node2)) => {
                let node1 = node1.borrow();
                let node2 = node2.borrow();
                node1.val == node2.val && 
                Self::is_same_tree(node1.left.clone(), node2.left.clone()) && 
                Self::is_same_tree(node1.right.clone(), node2.right.clone())
            },
            (None, None) => true,
            _ => false,
        }
    }
}


//Iterative solution

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back((p, q));

        while let Some((node1, node2)) = queue.pop_front() {
            match (node1, node2) {
                (Some(n1), Some(n2)) => {
                    if n1.borrow().val != n2.borrow().val {
                        return false;
                    }
                    queue.push_back((n1.borrow().left.clone(), n2.borrow().left.clone()));
                    queue.push_back((n1.borrow().right.clone(), n2.borrow().right.clone()));
                }
                (None, None) => continue,
                _ => return false,
            }
        }

        true
    }
}
