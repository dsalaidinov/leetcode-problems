Link: https://leetcode.com/problems/longest-palindromic-substring/

Algorithm: Manacher's algorithm
Time Complexity: O(n)
Space Complexity: O(n)

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut t = vec!['#'];
        for ch in s.chars() {
            t.push(ch);
            t.push('#');
        }
        
        let n = t.len();
        let mut p = vec![0; n];
        let (mut center, mut right) = (0, 0);
        let mut max_len = 0;
        let mut start = 0;
        
        for i in 0..n {
            if i < right {
                p[i] = p[2 * center - i].min(right - i);
            }
            
            while i + p[i] + 1 < n && i >= p[i] + 1 && t[i + p[i] + 1] == t[i - p[i] - 1] {
                p[i] += 1;
            }
            
            if i + p[i] > right {
                center = i;
                right = i + p[i];
            }
            
            if p[i] > max_len {
                max_len = p[i];
                start = (i - p[i]) / 2;
            }
        }
        
        s[start..start + max_len].to_string()
    }
}
