#![allow(dead_code)]
/*
59. 螺旋矩阵 II
中等
1.1K
相关企业
给你一个正整数 n ，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。



示例 1：


输入：n = 3
输出：[[1,2,3],[8,9,4],[7,6,5]]
示例 2：

输入：n = 1
输出：[[1]]


提示：

1 <= n <= 20
 */


struct Solution;

use std::iter;
// 定义方向
const DIRECTION: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

/*
首次移动 n个位置后切换方向，
之后2次移动n-1个位置后切换方向
n = 3时，分别在移动3,2,2,1,1次后切换方向
n = 4时，分别在移动4,3,3,2,2,1,1次后切换方向
 */
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        // 定义结果矩阵
        let mut ans = vec![vec![0; n as usize]; n as usize];
        // 移动距离迭代器 steps chain
        let chain = iter::once(n).chain(
            (1..n).rev().flat_map(|x| iter::repeat(x).take(2))
        );
        // 方向循环迭代
        let mut cycle = DIRECTION.iter().cycle();
        // 坐标初始化 填充数值n
        let (mut i, mut j, mut n) = (0, -1, 1..);
        for &(move_i, move_j) in
        chain.flat_map(|steps| iter::repeat(cycle.next().unwrap()).take(steps as usize))
        {
            i += move_i;
            j += move_j;
            ans[i as usize][j as usize] = n.next().unwrap();
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::generate_matrix::Solution;

    #[test]
    pub fn t1() {
        let i = Solution::generate_matrix(4);
        println!("{:?}", i);
    }
}