#![allow(dead_code)]
/*
85. 最大矩形
困难
1.6K
相关企业
给定一个仅包含 0 和 1 、大小为 rows x cols 的二维二进制矩阵，找出只包含 1 的最大矩形，并返回其面积。



示例 1：


输入：matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
输出：6
解释：最大矩形如上图所示。
示例 2：

输入：matrix = []
输出：0
示例 3：

输入：matrix = [["0"]]
输出：0
示例 4：

输入：matrix = [["1"]]
输出：1
示例 5：

输入：matrix = [["0","0"]]
输出：0


提示：

rows == matrix.length
cols == matrix[0].length
1 <= row, cols <= 200
matrix[i][j] 为 '0' 或 '1'
 */
struct Solution;
/*
动态规划
dp[i][j] = (wij,hij),
其中 hij是j列中以i行为截止点的连续的1的数量，或者说宽度为1的矩形的最大高

状态转移方程
如果 matrix[i][j] = 0 , dp[i][j] = 0

如果 matrix[i][j] = 1
dp[i][j] = (1 + dp[i][j - 1].0, 1 + dp[i - 1][j].1);

初始状态
dp[0][0] = 1 if matrix[0][0] = 1
dp[0][j] = 1 + dp[0][j-1] if matrix[0][j] = 1  高为1
dp[i][0] = 1 + dp[i-1][0] if matrix[i][0] = 1  宽为1
 */
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![(0, 0); n]; m];
        // 初始状态
        if matrix[0][0] == '1' {
            dp[0][0] = (1, 1);
        }
        for i in 1..m {
            if matrix[i][0] == '1' {
                dp[i][0] = (1, 1 + dp[i - 1][0].1);
            }
        }
        for j in 1..n {
            if matrix[0][j] == '1' {
                dp[0][j] = (1 + dp[0][j - 1].0, 1);
            }
        }
        // 状态转移计算
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == '1' {
                    dp[i][j] = (1 + dp[i][j - 1].0, 1 + dp[i - 1][j].1);
                }
            }
        }
        // println!("{:?}", dp);
        // 求出最大面积
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                let (w, mut h) = dp[i][j];
                ans = std::cmp::max(ans, h);
                for k in 1..w {
                    h = std::cmp::min(h, dp[i][j - k].1);
                    ans = std::cmp::max(ans, h * (k + 1));
                }
            }
        }
        ans as i32
    }
}


#[cfg(test)]
mod test {
    use crate::arr::maximal_rectangle::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::maximal_rectangle(
            vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ]
        );
        assert_eq!(ans, 6);
    }

    #[test]
    pub fn t2() {
        let ans = Solution::maximal_rectangle(
            vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '1', '1', '1'],
            ]
        );
        assert_eq!(ans, 9);
    }


    #[test]
    pub fn t3() {
        let ans = Solution::maximal_rectangle(
            vec![
                vec!['1', '0', '0', '1', '1'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '1'],
            ]
        );
        assert_eq!(ans, 8);
    }
}