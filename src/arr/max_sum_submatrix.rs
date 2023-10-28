#![allow(dead_code)]
/*
363. 矩形区域不超过 K 的最大数值和
困难
相关标签
相关企业
给你一个 m x n 的矩阵 matrix 和一个整数 k ，找出并返回矩阵内部矩形区域的不超过 k 的最大数值和。

题目数据保证总会存在一个数值和不超过 k 的矩形区域。



示例 1：


输入：matrix = [[1,0,1],[0,-2,3]], k = 2
输出：2
解释：蓝色边框圈出来的矩形区域 [[0, 1], [-2, 3]] 的数值和是 2，且 2 是不超过 k 的最大数字（k = 2）。
示例 2：

输入：matrix = [[2,2,-1]], k = 3
输出：3


提示：

m == matrix.length
n == matrix[i].length
1 <= m, n <= 100
-100 <= matrix[i][j] <= 100
-105 <= k <= 105


进阶：如果行数远大于列数，该如何设计解决方案？
 */

struct Solution;
/*
可以枚举矩阵的上边界(u)和下边界(d)，于是问题变成一维问题

给定一个整数数组和一个整数 k，计算该数组的最大区间和，要求区间和不超过 k。

使用前缀和处理
RS = S(r) - S(l) <= k
枚举有边界r，要使得RS尽可能大，则需要S(l)尽可能小
即找到最小的S(l)，其满足 S(l) >= S(r) - k


 */

use std::collections::BTreeSet;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = i32::MIN;
        let (m, n) = (matrix.len(), matrix[0].len());

        for u in 0..m { // 枚举上边界
            // 维护每一列的和
            let mut sum = vec![0; n];
            for d in u..m { // 枚举下边界
                for i in 0..n {
                    // 压缩为一维问题 更新截至d行的每列和
                    sum[i] += matrix[d][i];
                }
                let mut pre_sum = 0;
                let mut set = BTreeSet::new();
                set.insert(0);
                for &x in sum.iter() {
                    pre_sum += x;
                    if let Some(&left) = set.range(pre_sum - k..).next() {
                        ans = ans.max(pre_sum - left);
                    }
                    set.insert(pre_sum);
                }
            }
        }

        ans
    }
}