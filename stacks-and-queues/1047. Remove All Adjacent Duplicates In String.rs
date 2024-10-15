Link: https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();

        for c in s.chars() {
            if !stack.is_empty() && stack[stack.len()-1] == c {
                stack.pop();
            } else {
                stack.push(c);
            }

        }

        stack.into_iter().collect()
    }
}