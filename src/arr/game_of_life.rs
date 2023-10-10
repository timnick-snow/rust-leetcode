#![allow(dead_code)]
/*
289. 生命游戏
中等
相关标签
相关企业
根据 百度百科 ， 生命游戏 ，简称为 生命 ，是英国数学家约翰·何顿·康威在 1970 年发明的细胞自动机。

给定一个包含 m × n 个格子的面板，每一个格子都可以看成是一个细胞。每个细胞都具有一个初始状态： 1 即为 活细胞 （live），或 0 即为 死细胞 （dead）。每个细胞与其八个相邻位置（水平，垂直，对角线）的细胞都遵循以下四条生存定律：

如果活细胞周围八个位置的活细胞数少于两个，则该位置活细胞死亡；
如果活细胞周围八个位置有两个或三个活细胞，则该位置活细胞仍然存活；
如果活细胞周围八个位置有超过三个活细胞，则该位置活细胞死亡；
如果死细胞周围正好有三个活细胞，则该位置死细胞复活；
下一个状态是通过将上述规则同时应用于当前状态下的每个细胞所形成的，其中细胞的出生和死亡是同时发生的。给你 m x n 网格面板 board 的当前状态，返回下一个状态。



示例 1：


输入：board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
输出：[[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
示例 2：


输入：board = [[1,1],[1,0]]
输出：[[1,1],[1,1]]


提示：

m == board.length
n == board[i].length
1 <= m, n <= 25
board[i][j] 为 0 或 1


进阶：

你可以使用原地算法解决本题吗？请注意，面板上所有格子需要同时被更新：你不能先更新某些格子，然后使用它们的更新后的值再更新其他格子。
本题中，我们使用二维数组来表示面板。原则上，面板是无限的，但当活细胞侵占了面板边界时会造成问题。你将如何解决这些问题？
 */
struct Solution;
/*
定义值表示复合状态(过去，现在)
    过去   现在
00:  0     0
01:  1     0
10:  0     1
11:  1     1
 */

const LIVE_MASK: i32 = 1 << 1;

const ARR: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1),
];

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let (m, n) = (board.len(), board[0].len());
        let around = |(i, j): (usize, usize)| {
            let i = i as i32;
            let j = j as i32;
            let mut ans = Vec::new();
            for &(x, y) in ARR.iter() {
                let ix = i + x;
                let jy = j + y;
                if ix >= 0 && jy >= 0 && ix < m as i32 && jy < n as i32 {
                    ans.push((ix as usize, jy as usize));
                }
            }
            ans
        };

        // 计算状态
        for i in 0..m {
            for j in 0..n {
                let cnt = around((i, j)).iter()
                    .map(|&(x, y)| board[x][y])
                    .filter(|&live| (live & 1) == 1)
                    .count();
                if board[i][j] == 1 {
                    // 如果活细胞周围八个位置的活细胞数有2个或者3个，则该位置活细胞仍然存活；
                    if cnt == 2 || cnt == 3 {
                        // println!("{:?} 存活", (i, j));
                        board[i][j] |= LIVE_MASK;
                    }
                } else {
                    // 如果死细胞周围正好有三个活细胞，则该位置死细胞复活；
                    if cnt == 3 {
                        // println!("{:?} 复活了", (i, j));
                        board[i][j] |= LIVE_MASK;
                    }
                }
            }
        }


        // 将细胞更新为现在的状态
        for i in 0..m {
            for j in 0..n {
                board[i][j] >>= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::game_of_life::Solution;

    #[test]
    pub fn t1() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board);
        println!("result: \n{:?}", board);
    }
}