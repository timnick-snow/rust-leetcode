#![allow(dead_code)]
/*
417. 太平洋大西洋水流问题
中等
相关标签
相关企业
有一个 m × n 的矩形岛屿，与 太平洋 和 大西洋 相邻。 “太平洋” 处于大陆的左边界和上边界，而 “大西洋” 处于大陆的右边界和下边界。

这个岛被分割成一个由若干方形单元格组成的网格。给定一个 m x n 的整数矩阵 heights ， heights[r][c] 表示坐标 (r, c) 上单元格 高于海平面的高度 。

岛上雨水较多，如果相邻单元格的高度 小于或等于 当前单元格的高度，雨水可以直接向北、南、东、西流向相邻单元格。水可以从海洋附近的任何单元格流入海洋。

返回网格坐标 result 的 2D 列表 ，其中 result[i] = [ri, ci] 表示雨水从单元格 (ri, ci) 流动 既可流向太平洋也可流向大西洋 。



示例 1：



输入: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
输出: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
示例 2：

输入: heights = [[2,1],[1,2]]
输出: [[0,0],[0,1],[1,0],[1,1]]


提示：

m == heights.length
n == heights[r].length
1 <= m, n <= 200
0 <= heights[r][c] <= 105
 */
use std::collections::VecDeque;

struct Solution;
/*
从边界开始搜索  能流入大洋的标记

这里使用广度优先 BFS
 */
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (heights.len(), heights[0].len());
        // 流入太平洋
        let mut fp = vec![vec![false; n]; m];
        bfs(&heights, &mut fp, 0, 0);

        // 流入大西洋
        let mut fa = vec![vec![false; n]; m];
        bfs(&heights, &mut fa, m - 1, n - 1);

        let mut ans = Vec::new();
        for i in 0..m {
            for j in 0..n {
                // 同时能流入太平洋和大西洋则加入到结果集
                if fp[i][j] && fa[i][j] {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }
        ans
    }
}

// 方向定义
const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

// 高度 状态 行边界 列边界
fn bfs(heights: &Vec<Vec<i32>>, state: &mut Vec<Vec<bool>>, rb: usize, cb: usize) {
    let (m, n) = (state.len(), state[0].len());
    // 搜索队列
    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    // 行边界初始化
    for j in 0..n {
        deque.push_back((rb, j));
        state[rb][j] = true;
    }
    // 列边界初始化
    for i in 0..m {
        if !state[i][cb] {
            deque.push_back((i, cb));
            state[i][cb] = true;
        }
    }
    // bfs搜索
    while let Some((r, c)) = deque.pop_front() {
        // 枚举四个方向
        for &(di, dj) in DIR.iter() {
            let i = r as i32 + di;
            let j = c as i32 + dj;
            // 边界检查
            if i >= 0 && i < m as i32 && j >= 0 && j < n as i32 {
                let i = i as usize;
                let j = j as usize;
                // 可达格周围的单元格的高度要更高才能流入可达格
                if heights[i][j] >= heights[r][c] && !state[i][j] {
                    state[i][j] = true;
                    deque.push_back((i, j));
                }
            }
        }
    }
}