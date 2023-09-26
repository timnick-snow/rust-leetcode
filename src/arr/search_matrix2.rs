#![allow(dead_code)]
/*
240. 搜索二维矩阵 II
中等
相关标签
相关企业
编写一个高效的算法来搜索 m x n 矩阵 matrix 中的一个目标值 target 。该矩阵具有以下特性：

每行的元素从左到右升序排列。
每列的元素从上到下升序排列。


示例 1：


输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
输出：true
示例 2：


输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
输出：false


提示：

m == matrix.length
n == matrix[i].length
1 <= n, m <= 300
-109 <= matrix[i][j] <= 109
每行的所有元素从左到右升序排列
每列的所有元素从上到下升序排列
-109 <= target <= 109
 */
struct Solution;
/*
Z型查找

先搜索右上角 然后根据情况移动

 */
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut i, mut j) = (0, n - 1);
        while i < m {
            if matrix[i][j] == target {
                return true;
            }
            if matrix[i][j] > target {
                // 左移
                match j.checked_sub(1) {
                    None => { return false; }
                    Some(x) => j = x,
                }
            } else {
                // 下移
                i += 1;
            }
        }
        false
    }
}

