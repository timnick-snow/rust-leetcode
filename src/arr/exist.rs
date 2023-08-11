#![allow(dead_code)]
/*
79. 单词搜索
中等
1.6K
相关企业
给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。

单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。



示例 1：


输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
输出：true
示例 2：


输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
输出：true
示例 3：


输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
输出：false


提示：

m == board.length
n = board[i].length
1 <= m, n <= 6
1 <= word.length <= 15
board 和 word 仅由大小写英文字母组成


进阶：你可以使用搜索剪枝的技术来优化解决方案，使其在 board 更大的情况下可以更快解决问题？
 */
struct Solution;

/*
dfs(i,j,k) 表示以(i,j)为起始点检查board，能否满足word[k..]
 */
const DIRECTION: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        let mut visited = vec![vec![false; n]; m];
        let word = word.chars().collect::<Vec<char>>();
        for i in 0..m {
            for j in 0..n {
                visited[i][j] = true;
                if dfs(&board, &word, &mut visited, i, j, 0, m, n) {
                    return true;
                }
                visited[i][j] = false;
            }
        }
        false
    }
}

fn dfs(board: &Vec<Vec<char>>, word: &[char], visited: &mut Vec<Vec<bool>>,
       i: usize, j: usize, k: usize, m: usize, n: usize) -> bool {
    // 当前字符不满足
    if board[i][j] != word[k] {
        return false;
    }
    // 匹配完成
    if k == word.len() - 1 {
        return true;
    }
    // 下一个字符搜索各个可能的方向
    for &d in DIRECTION.iter() {
        let x = i as i32 + d.0;
        let y = j as i32 + d.1;
        if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 || visited[x as usize][y as usize] {
            continue;
        }
        let (x, y) = (x as usize, y as usize);
        // 访问(x,y)
        visited[x][y] = true;
        if dfs(board, word, visited, x, y, k + 1, m, n) {
            return true;
        }
        // 不满足 取消访问(x,y)
        visited[x][y] = false;
    }
    false
}


#[cfg(test)]
mod test {
    use crate::arr::exist::Solution;

    #[test]
    pub fn t1() {
        let solution = Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "SEE".into(),
        );
        assert!(solution);
    }


    #[test]
    pub fn t2() {
        let solution = Solution::exist(
            vec![
                vec!['A'],
            ],
            "A".into(),
        );
        assert!(solution);
    }
}


