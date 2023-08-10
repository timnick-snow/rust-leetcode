#![allow(dead_code)]
/*
73. 矩阵置零
提示
中等
908
相关企业
给定一个 m x n 的矩阵，如果一个元素为 0 ，则将其所在行和列的所有元素都设为 0 。请使用 原地 算法。



示例 1：


输入：matrix = [[1,1,1],[1,0,1],[1,1,1]]
输出：[[1,0,1],[0,0,0],[1,0,1]]
示例 2：


输入：matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
输出：[[0,0,0,0],[0,4,5,0],[0,3,1,0]]


提示：

m == matrix.length
n == matrix[0].length
1 <= m, n <= 200
-231 <= matrix[i][j] <= 231 - 1


进阶：

一个直观的解决方案是使用  O(mn) 的额外空间，但这并不是一个好的解决方案。
一个简单的改进方案是使用 O(m + n) 的额外空间，但这仍然不是最好的解决方案。
你能想出一个仅使用常量空间的解决方案吗？
 */
struct Solution;
/*
统计0的位置 保存到首行首列
如果matrix[i][j] = 0, 那么设置 matrix[i][0] = 0, matrix[0][j] = 0

进行置零操作，绕过首行首列
//
 */
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row_flag = false;
        let mut col_flag = false;
        let m = matrix.len();
        let n = matrix[0].len();
        // 计算 flag
        if matrix[0].iter().any(|&x| x == 0) {
            row_flag = true;
        }
        for i in 0..m {
            if matrix[i][0] == 0 {
                col_flag = true;
                break;
            }
        }
        // 统计0的位置 保存到首行首列
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        // 置零操作
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        // 处理首行首列
        if row_flag {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
        if col_flag {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}