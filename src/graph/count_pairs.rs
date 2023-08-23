#![allow(dead_code)]
/*
1782. 统计点对的数目
提示
困难
58
相关企业
给你一个无向图，无向图由整数 n  ，表示图中节点的数目，和 edges 组成，其中 edges[i] = [ui, vi] 表示 ui 和 vi 之间有一条无向边。同时给你一个代表查询的整数数组 queries 。

第 j 个查询的答案是满足如下条件的点对 (a, b) 的数目：

a < b
cnt 是与 a 或者 b 相连的边的数目，且 cnt 严格大于 queries[j] 。
请你返回一个数组 answers ，其中 answers.length == queries.length 且 answers[j] 是第 j 个查询的答案。

请注意，图中可能会有 重复边 。



示例 1：


输入：n = 4, edges = [[1,2],[2,4],[1,3],[2,3],[2,1]], queries = [2,3]
输出：[6,5]
解释：每个点对中，与至少一个点相连的边的数目如上图所示。
示例 2：

输入：n = 5, edges = [[1,5],[1,5],[3,4],[2,5],[1,3],[5,1],[2,3],[2,5]], queries = [1,2,3,4,5]
输出：[10,10,9,8,6]


提示：

2 <= n <= 2 * 104
1 <= edges.length <= 105
1 <= ui, vi <= n
ui != vi
1 <= queries.length <= 20
0 <= queries[j] < edges.length
 */
use std::collections::HashMap;

struct Solution;

/*
cnt(a,b) = degree[a] + degree[b] - overlap(a,b)
其中：
degree[a] 是节点a的度，即与节点a相连的边的数目
degree[b] 是节点b的度
overlap(a,b) 是节点a,b之间的边的数量


首先求出 degree[a] + degree[b] > query，可以对 degree数组先进行排序。
则问题等价于在有序数组中，求出所有数字对的数目，使得它们的和大于给定的值。这是一个经典的双指针问题

对于节点a,b之间的边的数量，可以将边ab映射到一个整数 a*(n+1) + b, (a < b <= n)
 */
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut degree = vec![0; n];
        let mut overlap = HashMap::new();
        // 统计各节点的度 两节点之间的边的数量
        for pair in edges.iter() {
            let (mut a, mut b) = (pair[0] as usize - 1, pair[1] as usize - 1);
            // 确保 a<b
            if a > b {
                let temp = a;
                a = b;
                b = temp;
            }
            degree[a] += 1;
            degree[b] += 1;
            overlap.entry(a * (n + 1) + b)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        // 排序 为了保留能查询到指定节点的度的功能 另开一个数组
        let mut arr = degree.clone();
        arr.sort_unstable();
        // 开始计算ans
        let mut ans = Vec::new();
        for query in queries.into_iter() {
            let mut cnt = 0;
            // 双指针计算
            let (mut i, mut j) = (0, n - 1);
            while i < n {
                while j > i && arr[i] + arr[j] > query {
                    j -= 1;
                }
                cnt += n - 1 - std::cmp::max(i, j);
                i += 1;
            }
            // 求出重复计算
            for (&val, &freq) in overlap.iter() {
                let (a, b) = (val / (n + 1), val % (n + 1));
                if degree[a] + degree[b] > query && degree[a] + degree[b] - freq <= query {
                    cnt -= 1;
                }
            }
            ans.push(cnt as i32);
        }
        ans
    }
}