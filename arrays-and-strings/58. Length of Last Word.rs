Link: https://leetcode.com/problems/length-of-last-word/

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<_> = s.split_whitespace().collect();
        
        if let Some(last_word) = words.last() {
            return last_word.len() as i32;
        }
        0
    }
}