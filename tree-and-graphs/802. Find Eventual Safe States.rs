Link: https://leetcode.com/problems/find-eventual-safe-states/?envType=study-plan-v2&envId=graph-theory

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut state = vec![0; n];
        let mut result = vec![];

        for i in 0..n {
            if Solution::dfs(i, &graph, &mut state) == 2 {
                result.push(i as i32);
            }
        }

        result
    }

    pub fn dfs(node: usize, graph: &Vec<Vec<i32>>, state: &mut Vec<i32>) -> i32 {
        if state[node] == 2 {
            return 2;
        }
        if state[node] == 1 {
            return 1;
        }

        state[node] = 1;

        for &neighbor in &graph[node] {
            if state[neighbor as usize] == 1 || Solution::dfs(neighbor as usize, graph, state) == 1 {
                return 1;
            }
        }

        state[node] = 2;
        2
    }
}
