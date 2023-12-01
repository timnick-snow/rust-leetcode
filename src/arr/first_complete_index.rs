#![allow(dead_code)]
/*
2661. 找出叠涂元素
中等
相关标签
相关企业
提示
给你一个下标从 0 开始的整数数组 arr 和一个 m x n 的整数 矩阵 mat 。arr 和 mat 都包含范围 [1，m * n] 内的 所有 整数。

从下标 0 开始遍历 arr 中的每个下标 i ，并将包含整数 arr[i] 的 mat 单元格涂色。

请你找出 arr 中在 mat 的某一行或某一列上都被涂色且下标最小的元素，并返回其下标 i 。



示例 1：

image explanation for example 1
输入：arr = [1,3,4,2], mat = [[1,4],[2,3]]
输出：2
解释：遍历如上图所示，arr[2] 在矩阵中的第一行或第二列上都被涂色。
示例 2：

image explanation for example 2
输入：arr = [2,8,7,4,1,3,5,6,9], mat = [[3,2,5],[1,4,6],[8,7,9]]
输出：3
解释：遍历如上图所示，arr[3] 在矩阵中的第二列上都被涂色。


提示：

m == mat.length
n = mat[i].length
arr.length == m * n
1 <= m, n <= 105
1 <= m * n <= 105
1 <= arr[i], mat[r][c] <= m * n
arr 中的所有整数 互不相同
mat 中的所有整数 互不相同
 */
struct Solution;
/*
啊啊啊
直接对每行每列进行涂色计数就行了
 */
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut row = vec![vec![(0, 0); n]; m];
        let mut col = vec![vec![(0, 0); n]; m];

        let mut idx = vec![(0, 0); m * n];
        for i in 0..m {
            for j in 0..n {
                idx[(mat[i][j] - 1) as usize] = (i, j);
            }
        }

        for (index, x) in arr.into_iter().enumerate() {
            // 涂色(i,j)
            let (i, j) = idx[(x - 1) as usize];
            // check row
            let left = if j == 0 || row[i][j - 1].0 == row[i][j - 1].1 {
                j as i32 - 1
            } else {
                row[i][j - 1].0
            };
            let right = if j == n - 1 || row[i][j + 1].0 == row[i][j + 1].1 {
                j as i32 + 1
            } else {
                row[i][j + 1].1
            };
            row[i][(left + 1) as usize].1 = right;
            row[i][(right - 1) as usize].0 = left;
            row[i][j] = (left, right);
            if left == -1 && right == n as i32 {
                return index as i32;
            }

            // check col
            let up = if i == 0 || col[i - 1][j].0 == col[i - 1][j].1 {
                i as i32 - 1
            } else {
                col[i - 1][j].0
            };
            let down = if i == m - 1 || col[i + 1][j].0 == col[i + 1][j].1 {
                i as i32 + 1
            } else {
                col[i + 1][j].1
            };
            col[(up + 1) as usize][j].1 = down;
            col[(down - 1) as usize][j].0 = up;
            col[i][j] = (up, down);
            if up == -1 && down == m as i32 {
                return index as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::arr::first_complete_index::Solution;

    #[test]
    pub fn t1() {
        let arr = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
        let mat = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
        let ans = Solution::first_complete_index(arr, mat);
        assert_eq!(ans, 3);
    }

    #[test]
    pub fn t2() {
        let arr = vec![9, 7, 12, 16, 17, 3, 14, 5, 8, 10, 2, 19, 13, 15, 20, 18, 4, 6, 11, 1];
        let mat = vec![vec![20, 19, 1, 3, 10], vec![17, 11, 14, 9, 7],
                       vec![12, 2, 15, 6, 16], vec![4, 8, 13, 18, 5]];
        let ans = Solution::first_complete_index(arr, mat);
        assert_eq!(ans, 9);
    }
}