Link: https://leetcode.com/problems/removing-stars-from-a-string/

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '*' => {
                    stack.pop();
                }
                _ => {
                    stack.push(c);
                }
            }

        }

        stack.into_iter().collect()
    }
}