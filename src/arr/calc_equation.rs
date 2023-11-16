#![allow(dead_code)]
/*
399. 除法求值
中等
相关标签
相关企业
提示
给你一个变量对数组 equations 和一个实数值数组 values 作为已知条件，其中 equations[i] = [Ai, Bi] 和 values[i] 共同表示等式 Ai / Bi = values[i] 。每个 Ai 或 Bi 是一个表示单个变量的字符串。

另有一些以数组 queries 表示的问题，其中 queries[j] = [Cj, Dj] 表示第 j 个问题，请你根据已知条件找出 Cj / Dj = ? 的结果作为答案。

返回 所有问题的答案 。如果存在某个无法确定的答案，则用 -1.0 替代这个答案。如果问题中出现了给定的已知条件中没有出现的字符串，也需要用 -1.0 替代这个答案。

注意：输入总是有效的。你可以假设除法运算中不会出现除数为 0 的情况，且不存在任何矛盾的结果。

注意：未在等式列表中出现的变量是未定义的，因此无法确定它们的答案。



示例 1：

输入：equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
输出：[6.00000,0.50000,-1.00000,1.00000,-1.00000]
解释：
条件：a / b = 2.0, b / c = 3.0
问题：a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
结果：[6.0, 0.5, -1.0, 1.0, -1.0 ]
注意：x 是未定义的 => -1.0
示例 2：

输入：equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
输出：[3.75000,0.40000,5.00000,0.20000]
示例 3：

输入：equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
输出：[0.50000,2.00000,-1.00000,-1.00000]


提示：

1 <= equations.length <= 20
equations[i].length == 2
1 <= Ai.length, Bi.length <= 5
values.length == equations.length
0.0 < values[i] <= 20.0
1 <= queries.length <= 20
queries[i].length == 2
1 <= Cj.length, Dj.length <= 5
Ai, Bi, Cj, Dj 由小写英文字母与数字组成
 */
use std::collections::HashMap;

struct Solution;
/*
并查集的运用
 */
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let eq_size = equations.len();
        let mut union_find = UnionFind::new(eq_size << 1);
        // 预处理 变量映射+并查集合并
        let mut map: HashMap<String, usize> = HashMap::with_capacity(eq_size << 1);
        let mut id = 0;
        for (i, mut eq) in equations.into_iter().enumerate() {
            let y = eq.pop().unwrap();
            let x = eq.pop().unwrap();

            let idx;
            if !map.contains_key(&x) {
                map.insert(x, id);
                idx = id;
                id += 1;
            } else {
                idx = *map.get(&x).unwrap();
            }

            let idy;
            if !map.contains_key(&y) {
                map.insert(y, id);
                idy = id;
                id += 1;
            } else {
                idy = *map.get(&y).unwrap();
            }
            // 合并
            union_find.union(idx, idy, values[i]);
        }

        // 查询
        let query_size = queries.len();
        let mut ans = vec![0.0; query_size];
        for (i, mut query) in queries.into_iter().enumerate() {
            let y = query.pop().unwrap();
            let x = query.pop().unwrap();

            let idx = *map.get(&x).unwrap_or(&id);
            let idy = *map.get(&y).unwrap_or(&id);

            if idx == id || idy == id {
                ans[i] = -1.0;
            } else {
                ans[i] = union_find.connected_weight(idx, idy);
            }
        }
        ans
    }
}

struct UnionFind {
    parent: Vec<usize>,
    weight: Vec<f64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind {
            parent,
            weight: vec![1.0; n],
        }
    }

    /// 查找指定节点的根节点并进行路径压缩
    /// x 需要查找的节点
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let old_parent = self.parent[x];
            self.parent[x] = self.find(old_parent);
            self.weight[x] *= self.weight[old_parent];
        }
        self.parent[x]
    }

    /// 求两个节点的连接权重 不相连返回-1
    fn connected_weight(&mut self, x: usize, y: usize) -> f64 {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            self.weight[x] / self.weight[y]
        } else {
            -1.0
        }
    }

    /// 合并两个分量
    fn union(&mut self, x: usize, y: usize, val: f64) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        self.parent[root_x] = root_y;
        self.weight[root_x] = self.weight[y] * val / self.weight[x];
    }
}