Link: https://leetcode.com/problems/number-of-recent-calls/
use std::collections::VecDeque;

struct RecentCounter {
    queue: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        &self.queue.push_back(t);

        while let Some(&front) = &self.queue.front() {
            if front < t - 3000 {
                &self.queue.pop_front();
            } else {
                break;
            }
        }

        self.queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */