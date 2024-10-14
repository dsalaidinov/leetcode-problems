Link: https://leetcode.com/problems/valid-palindrome/

Time: O(n)
Space: O(n)

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered: Vec<char> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

        let n = filtered.len();

        for i in 0..n / 2 {
            if filtered[i] != filtered[n - i - 1] {
                return false;
            }
        }

        true
    }
}

Time: O(n)
Space: O(1)

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut right = bytes.len().wrapping_sub(1);

        while left < right {
            while left < right && !bytes[left].is_ascii_alphanumeric() {
                left += 1;
            }
            while left < right && !bytes[right].is_ascii_alphanumeric() {
                right -= 1;
            }

            if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}
