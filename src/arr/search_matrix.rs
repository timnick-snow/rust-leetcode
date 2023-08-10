#![allow(dead_code)]
/*
74. 搜索二维矩阵
中等
826
相关企业
给你一个满足下述两条属性的 m x n 整数矩阵：

每行中的整数从左到右按非递减顺序排列。
每行的第一个整数大于前一行的最后一个整数。
给你一个整数 target ，如果 target 在矩阵中，返回 true ；否则，返回 false 。



示例 1：


输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
输出：true
示例 2：


输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
输出：false


提示：

m == matrix.length
n == matrix[i].length
1 <= m, n <= 100
-104 <= matrix[i][j], target <= 104
 */
struct Solution;
/*
二分法查找
只是线性数组变成了二维数组
对于 m*n 的矩阵， 其线性索引范围等价于[0, m*n-1]

对于位于线性索引范围index处，将其转换为矩阵索引
i = index / n
j = index % n
 */
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = 0;
        let mut right = m * n - 1;
        while left <= right {
            let mid = (left + right) >> 1;
            let value = matrix[mid / n][mid % n];
            if value == target {
                return true;
            }
            if value < target {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return false;
                }
                right = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use crate::arr::search_matrix::Solution;

    #[test]
    fn t1() {
        let ans = Solution::search_matrix(
            vec![vec![1]], 0,
        );
        assert_eq!(ans, false);
    }
}