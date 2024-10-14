Link: https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set = HashSet::new();
        let (mut left, mut max_len) = (0, 0);
        let chars: Vec<char> = s.chars().collect();

        for right in 0..chars.len() {
            while char_set.contains(&chars[right]) {
                char_set.remove(&chars[left]);
                left += 1;
            }
            char_set.insert(chars[right]);
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}