#![allow(dead_code)]
/*
310. 最小高度树
中等
相关标签
相关企业
提示
树是一个无向图，其中任何两个顶点只通过一条路径连接。 换句话说，一个任何没有简单环路的连通图都是一棵树。

给你一棵包含 n 个节点的树，标记为 0 到 n - 1 。给定数字 n 和一个有 n - 1 条无向边的 edges 列表（每一个边都是一对标签），其中 edges[i] = [ai, bi] 表示树中节点 ai 和 bi 之间存在一条无向边。

可选择树中任何一个节点作为根。当选择节点 x 作为根节点时，设结果树的高度为 h 。在所有可能的树中，具有最小高度的树（即，min(h)）被称为 最小高度树 。

请你找到所有的 最小高度树 并按 任意顺序 返回它们的根节点标签列表。

树的 高度 是指根节点和叶子节点之间最长向下路径上边的数量。


示例 1：


输入：n = 4, edges = [[1,0],[1,2],[1,3]]
输出：[1]
解释：如图所示，当根是标签为 1 的节点时，树的高度是 1 ，这是唯一的最小高度树。
示例 2：


输入：n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
输出：[3,4]


提示：

1 <= n <= 2 * 104
edges.length == n - 1
0 <= ai, bi < n
ai != bi
所有 (ai, bi) 互不相同
给定的输入 保证 是一棵树，并且 不会有重复的边
 */
use std::collections::{HashMap, VecDeque};

struct Solution;

/*
两个相距最远的点的中点就是所求的根节点

使用拓扑排序求出这个中点（中点可能有2个，当相距最远的两个点的线上有偶数个节点时）
求出所有节点的入度 degree[i], 度为1的节点为叶子节点，移除这些节点，并将与这些节点相连的点的度-1
重复这个过程，直到最后只剩下1或2个节点


 */
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n < 3 {
            return (0..n).collect();
        }
        let mut degree = vec![0; n as usize];
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for x in edges.iter() {
            let (a, b) = (x[0], x[1]);
            degree[a as usize] += 1;
            degree[b as usize] += 1;
            map.entry(a).or_default().push(b);
            map.entry(b).or_default().push(a);
        }
        // 初始化队列 将初始入度为1的节点加入
        let mut deque = VecDeque::new();
        for i in 0..n as usize {
            if degree[i] == 1 {
                deque.push_back(i as i32);
            }
        }
        // 进行叶子节点移出  一轮一轮的进行
        let mut rest_num = n as usize;
        while rest_num > 2 {
            let size = deque.len();
            rest_num -= size;
            for _ in 0..size {
                let x = deque.pop_front().unwrap();
                for &y in map.get(&x).unwrap() {
                    // 与x相连的节点度-1
                    degree[y as usize] -= 1;
                    if degree[y as usize] == 1 {
                        deque.push_back(y);
                    }
                }
            }
        }
        deque.into_iter().collect()
    }
}