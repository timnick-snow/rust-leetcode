#![allow(dead_code)]
/*
48. 旋转图像
中等
1.7K
相关企业
给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。

你必须在 原地 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。

示例 1：

输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
输出：[[7,4,1],[8,5,2],[9,6,3]]
示例 2：

输入：matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
输出：[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]

提示：

n == matrix.length == matrix[i].length
1 <= n <= 20
-1000 <= matrix[i][j] <= 1000
*/

struct Solution;
/*
由外向内不断旋转

(0,0) (0,1) (0,2) (0,3) .
(1,0) (1,1) (1,2) (1,3) .
(2,0) (2,1) (2,2) (2,3) .
(3,0) (3,1) (3,2) (3,3) .
.       .     .     .   .
第i行 变成了 第n-i列
第j列 变成了 第j行

映射关系
(i,j) => (j,n-1-i)
 */
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let transfer = |i: &mut usize, j: &mut usize| {
            let temp = *i;
            *i = *j;
            *j = n - 1 - temp;
        };
        for layer in 0..n >> 1 {
            let (mut start, end) = (layer, n - 1 - layer);
            while start < end {
                // 当前要处理的数
                let mut cur = matrix[layer][start];
                // 当前处理的数所在坐标
                let (mut i, mut j) = (layer, start);
                for _ in 0..4 {
                    // print!("{:?} ", (i, j));
                    transfer(&mut i, &mut j);
                    // println!("=> {:?} ", (i, j));
                    let temp = matrix[i][j];
                    matrix[i][j] = cur;
                    cur = temp;
                }
                start += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::rotate::Solution;

    #[test]
    fn t1() {
        let mut vec1 = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        Solution::rotate(&mut vec1);
        println!("{:?}", vec1);
    }
}