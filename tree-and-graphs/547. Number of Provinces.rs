Link: https://leetcode.com/problems/number-of-provinces/?envType=study-plan-v2&envId=graph-theory

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![false; is_connected.len()];
        let mut provinces = 0;

        for i in 0..is_connected.len() {
            if !visited[i] {
                provinces += 1;
                Solution::dfs(&is_connected, i, &mut visited);
            }
        }

        provinces
    }

    pub fn dfs(is_connected: &Vec<Vec<i32>>, city: usize, visited: &mut Vec<bool>) {
        visited[city] = true;

        for i in 0..is_connected.len() {
            if is_connected[city][i] == 1 && !visited[i] {
                 Solution::dfs(is_connected, i, visited);
            }
        }
    }

    
}