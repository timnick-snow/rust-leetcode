#![allow(dead_code)]
/*
200. 岛屿数量
中等
2.3K
相关企业
给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。

岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。

此外，你可以假设该网格的四条边均被水包围。



示例 1：

输入：grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
输出：1
示例 2：

输入：grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
输出：3


提示：

m == grid.length
n == grid[i].length
1 <= m, n <= 300
grid[i][j] 的值为 '0' 或 '1'
 */
struct Solution;
/*
dfs 将相邻的陆地标记
 */
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    ans += 1;
                    expand(&mut grid, i, j, m, n);
                }
            }
        }
        ans
    }
}

fn expand(grid: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
    if i > m - 1 || j > n - 1 || grid[i][j] != '1' {
        return;
    }
    grid[i][j] = '#';
    if j > 0 {
        expand(grid, i, j - 1, m, n);
    }
    expand(grid, i, j + 1, m, n);
    if i > 0 {
        expand(grid, i - 1, j, m, n);
    }
    expand(grid, i + 1, j, m, n);
}

#[cfg(test)]
mod test {
    use crate::arr::num_islands::Solution;

    #[test]
    pub fn t1() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ];
        let ans = Solution::num_islands(grid);
        assert_eq!(ans, 3);
    }
}