#![allow(dead_code)]

/*
304. 二维区域和检索 - 矩阵不可变
中等
相关标签
相关企业
给定一个二维矩阵 matrix，以下类型的多个请求：

计算其子矩形范围内元素的总和，该子矩阵的 左上角 为 (row1, col1) ，右下角 为 (row2, col2) 。
实现 NumMatrix 类：

NumMatrix(int[][] matrix) 给定整数矩阵 matrix 进行初始化
int sumRegion(int row1, int col1, int row2, int col2) 返回 左上角 (row1, col1) 、右下角 (row2, col2) 所描述的子矩阵的元素 总和 。


示例 1：



输入:
["NumMatrix","sumRegion","sumRegion","sumRegion"]
[[[[3,0,1,4,2],[5,6,3,2,1],[1,2,0,1,5],[4,1,0,1,7],[1,0,3,0,5]]],[2,1,4,3],[1,1,2,2],[1,2,2,4]]
输出:
[null, 8, 11, 12]

解释:
NumMatrix numMatrix = new NumMatrix([[3,0,1,4,2],[5,6,3,2,1],[1,2,0,1,5],[4,1,0,1,7],[1,0,3,0,5]]);
numMatrix.sumRegion(2, 1, 4, 3); // return 8 (红色矩形框的元素总和)
numMatrix.sumRegion(1, 1, 2, 2); // return 11 (绿色矩形框的元素总和)
numMatrix.sumRegion(1, 2, 2, 4); // return 12 (蓝色矩形框的元素总和)

提示：

m == matrix.length
n == matrix[i].length
1 <= m, n <= 200
-105 <= matrix[i][j] <= 105
0 <= row1 <= row2 < m
0 <= col1 <= col2 < n
最多调用 104 次 sumRegion 方法
 */

/*
继续沿用前缀和的思想
我们记录以(0,0)为左上角，(i,j)为右下角的矩阵区域和

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
(0,0)   |                       |
   S3   |         S2            |
        |                       |
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        |(r1,c1)                |
        |                       |
  S1    |                       |
        |                       |
        |               (r2,c2) |
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~




那么区域矩形和 S = S0 - (S1+S3) - (S2+S3) + S3

S = dp[r2+1][c2+1] - dp[r2+1][c1] - dp[r1][c2+1] + dp[r1][c1]


如何求原点到当前点的矩形区域和呢?
dp[i][j] = dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1] + matrix[i][j]

 */
struct NumMatrix {
    dp: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1] + matrix[i - 1][j - 1];
            }
        }
        NumMatrix {
            dp
        }
    }

    fn sum_region(&self, r1: i32, c1: i32, r2: i32, c2: i32) -> i32 {
        let (r1, c1, r2, c2) = (r1 as usize, c1 as usize, r2 as usize, c2 as usize);
        self.dp[r2 + 1][c2 + 1] - self.dp[r2 + 1][c1] - self.dp[r1][c2 + 1] + self.dp[r1][c1]
    }
}

/*
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */