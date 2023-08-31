#![allow(dead_code)]
/*
1761. 一个图中连通三元组的最小度数
提示
困难
40
相关企业
给你一个无向图，整数 n 表示图中节点的数目，edges 数组表示图中的边，其中 edges[i] = [ui, vi] ，表示 ui 和 vi 之间有一条无向边。

一个 连通三元组 指的是 三个 节点组成的集合且这三个点之间 两两 有边。

连通三元组的度数 是所有满足此条件的边的数目：一个顶点在这个三元组内，而另一个顶点不在这个三元组内。

请你返回所有连通三元组中度数的 最小值 ，如果图中没有连通三元组，那么返回 -1 。



示例 1：


输入：n = 6, edges = [[1,2],[1,3],[3,2],[4,1],[5,2],[3,6]]
输出：3
解释：只有一个三元组 [1,2,3] 。构成度数的边在上图中已被加粗。
示例 2：


输入：n = 7, edges = [[1,3],[4,1],[4,3],[2,5],[5,6],[6,7],[7,5],[2,6]]
输出：0
解释：有 3 个三元组：
1) [1,4,3]，度数为 0 。
2) [2,5,6]，度数为 2 。
3) [5,6,7]，度数为 2 。


提示：

2 <= n <= 400
edges[i].length == 2
1 <= edges.length <= n * (n-1) / 2
1 <= ui, vi <= n
ui != vi
图中没有重复的边。
 */
struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
        for edge in edges.into_iter() {
            let (a, b) = (edge[0], edge[1]);
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);
        }
        let mut ans = None;
        let empty = &HashSet::new();
        for first in 1..=n {
            let second_set = map.get(&first).unwrap_or(empty);
            for &second in second_set.iter() {
                if second < first { continue; }
                let third_set = map.get(&second).unwrap_or(empty);
                for &third in third_set {
                    if third < second { continue; }
                    let first_set = map.get(&third).unwrap_or(empty);
                    if first_set.contains(&first) {
                        // 找到三元组
                        let deg = first_set.len() + second_set.len() + third_set.len() - 6;
                        ans = Some(std::cmp::min(ans.unwrap_or(i32::MAX), deg as i32));
                    }
                }
            }
        }
        ans.unwrap_or(-1)
    }
}