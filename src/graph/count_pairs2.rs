#![allow(dead_code)]
/*
2316. 统计无向图中无法互相到达点对数
中等
相关标签
相关企业
提示
给你一个整数 n ，表示一张 无向图 中有 n 个节点，编号为 0 到 n - 1 。同时给你一个二维整数数组 edges ，其中 edges[i] = [ai, bi] 表示节点 ai 和 bi 之间有一条 无向 边。

请你返回 无法互相到达 的不同 点对数目 。



示例 1：



输入：n = 3, edges = [[0,1],[0,2],[1,2]]
输出：0
解释：所有点都能互相到达，意味着没有点对无法互相到达，所以我们返回 0 。
示例 2：



输入：n = 7, edges = [[0,2],[0,5],[2,4],[1,6],[5,4]]
输出：14
解释：总共有 14 个点对互相无法到达：
[[0,1],[0,3],[0,6],[1,2],[1,3],[1,4],[1,5],[2,3],[2,6],[3,4],[3,5],[3,6],[4,6],[5,6]]
所以我们返回 14 。


提示：

1 <= n <= 105
0 <= edges.length <= 2 * 105
edges[i].length == 2
0 <= ai, bi < n
ai != bi
不会有重复边。
 */
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut map: Vec<Vec<i32>> = vec![vec![]; n];
        for edge in edges.into_iter() {
            let (a, b) = (edge[0], edge[1]);
            map[a as usize].push(b);
            map[b as usize].push(a);
        }

        let mut visited = vec![0; n];
        let mut graph = Vec::new();
        let mut deque = VecDeque::new();
        for i in 0..n {
            if visited[i] == 0 {
                // BFS 统计与i相连接的节点的数量  此图中节点数
                let mut cnt = 0;
                deque.push_back(i);
                while let Some(node) = deque.pop_front() {
                    if visited[node]==0 {
                        // 标记为已访问
                        visited[node] = 1;
                        cnt += 1;
                        for &x in map[node].iter() {
                            if visited[x as usize] == 0 {
                                deque.push_back(x as usize);
                            }
                        }
                    }
                }
                graph.push(cnt);
            }
        }
        // 统计节点对数
        let mut sum = 0;
        let mut ans = 0;
        for x in graph {
            ans += sum * x as i64;
            sum += x;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::graph::count_pairs2::Solution;

    #[test]
    pub fn t1() {
        let n = 7;
        let edges = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];
        let pairs = Solution::count_pairs(n, edges);
        println!("{}", pairs);
    }
}