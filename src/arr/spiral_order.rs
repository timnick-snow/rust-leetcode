#![allow(dead_code)]
/*
54. 螺旋矩阵
提示
中等
1.4K
相关企业
给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。



示例 1：


输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
输出：[1,2,3,6,9,8,7,4,5]
示例 2：


输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
输出：[1,2,3,4,8,12,11,10,9,5,6,7]


提示：

m == matrix.length
n == matrix[i].length
1 <= m, n <= 10
-100 <= matrix[i][j] <= 100
 */

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = Vec::new();
        let direction = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut left, mut top, mut right, mut bottom) = (-1, -1, n as i32, m as i32);
        let (mut i, mut j) = (0_i32, 0_i32);
        let mut change = direction[0];
        for _ in 0..m * n {
            ans.push(matrix[i as usize][j as usize]);
            if j + change.1 >= right {
                change = direction[1];
                top += 1;
            }
            if i + change.0 >= bottom {
                change = direction[2];
                right -= 1;
            }
            if j + change.1 <= left {
                change = direction[3];
                bottom -= 1;
            }
            if i + change.0 <= top {
                change = direction[0];
                left += 1;
            }
            // println!("{:?} + {:?}", (i, j), change);
            // (i, j) = (i + change.0, j + change.1);
            i = i + change.0;
            j = j + change.1;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::spiral_order::Solution;

    #[test]
    fn t1() {
        // matrix = [[1,2,3],[4,5,6],[7,8,9]]
        let vec1 = Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ]);
        println!("{:?}", vec1);
    }
}