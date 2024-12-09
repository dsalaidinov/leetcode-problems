Link: https://leetcode.com/problems/keys-and-rooms/?envType=study-plan-v2&envId=graph-theory

use std::collections::HashSet;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
          let mut visited: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = Vec::new();

        stack.push(0);
        visited.insert(0);

         while !stack.is_empty() { 
            if let Some(room) = stack.pop() {
                
                for &neighbor in rooms[room as usize].iter() {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        stack.push(neighbor);
                    }
                }
            }

         }

         return visited.len() == rooms.len();
    }
}