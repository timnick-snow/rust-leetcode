#![allow(dead_code)]
/*
2258. 逃离火灾
困难
相关标签
相关企业
提示
给你一个下标从 0 开始大小为 m x n 的二维整数数组 grid ，它表示一个网格图。每个格子为下面 3 个值之一：

0 表示草地。
1 表示着火的格子。
2 表示一座墙，你跟火都不能通过这个格子。
一开始你在最左上角的格子 (0, 0) ，你想要到达最右下角的安全屋格子 (m - 1, n - 1) 。每一分钟，你可以移动到 相邻 的草地格子。每次你移动 之后 ，着火的格子会扩散到所有不是墙的 相邻 格子。

请你返回你在初始位置可以停留的 最多 分钟数，且停留完这段时间后你还能安全到达安全屋。如果无法实现，请你返回 -1 。如果不管你在初始位置停留多久，你 总是 能到达安全屋，请你返回 109 。

注意，如果你到达安全屋后，火马上到了安全屋，这视为你能够安全到达安全屋。

如果两个格子有共同边，那么它们为 相邻 格子。



示例 1：



输入：grid = [[0,2,0,0,0,0,0],[0,0,0,2,2,1,0],[0,2,0,0,1,2,0],[0,0,2,2,2,0,2],[0,0,0,0,0,0,0]]
输出：3
解释：上图展示了你在初始位置停留 3 分钟后的情形。
你仍然可以安全到达安全屋。
停留超过 3 分钟会让你无法安全到达安全屋。
示例 2：



输入：grid = [[0,0,0,0],[0,1,2,0],[0,2,0,0]]
输出：-1
解释：上图展示了你马上开始朝安全屋移动的情形。
火会蔓延到你可以移动的所有格子，所以无法安全到达安全屋。
所以返回 -1 。
示例 3：



输入：grid = [[0,0,0],[2,2,0],[1,2,0]]
输出：1000000000
解释：上图展示了初始网格图。
注意，由于火被墙围了起来，所以无论如何你都能安全到达安全屋。
所以返回 109 。


提示：

m == grid.length
n == grid[i].length
2 <= m, n <= 300
4 <= m * n <= 2 * 104
grid[i][j] 是 0 ，1 或者 2 。
grid[0][0] == grid[m - 1][n - 1] == 0
 */
use std::collections::{HashSet, VecDeque};

struct Solution;
/*
使用多源BFS来找到火势到达每个格子的的最早时间

然后，从给定 t 开始，判断是否有通往安全屋的安全路径。
使用二分搜索来有效地找到允许我们到达安全屋的最大 t。
 */

const SAFE_TIME: i32 = 1_000_000_000;
const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

impl Solution {
    pub fn maximum_minutes(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        // 每个格子的着火时间
        let mut fire_time = vec![vec![i32::MAX; n]; m];
        // 寻找记录火源位置
        let mut fire_set = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    fire_set.insert((i, j));
                }
            }
        }

        // 多元BFS
        fire_set.into_iter()
            .for_each(|ceil| bfs_fire_time(&grid, ceil, &mut fire_time));

        if fire_time[0][0] == 0 || fire_time[m - 1][n - 1] == 0 {
            return -1;
        }
        println!("{:?}", fire_time);
        // 二分搜索
        let mut left = 0;
        let mut right = fire_time[0][0].min(fire_time[m - 1][n - 1]);
        while left <= right {
            let mid = left + (right - left) / 2;
            if check_path(mid, &mut grid, &fire_time) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right.min(SAFE_TIME)
    }
}

fn check_path(time: i32, grid: &mut Vec<Vec<i32>>, fire_time: &Vec<Vec<i32>>) -> bool {
    // dfs_check(time, (0, 0), grid, fire_time)
    let (m, n) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; n]; m];
    let mut deque = VecDeque::new();
    deque.push_back((0, 0, time));
    visited[0][0] = true;

    while let Some((x, y, t)) = deque.pop_front() {
        for &(dx, dy) in DIRS.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < (m as i32) && ny >= 0 && ny < (n as i32) {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] != 0 || visited[nx][ny] {
                    continue;
                }
                // 到达安全屋
                if nx == m - 1 && ny == n - 1 {
                    return fire_time[nx][ny] >= t + 1;
                }
                // 未到达安全屋
                if fire_time[nx][ny] > t + 1 {
                    deque.push_back((nx, ny, t + 1));
                    visited[nx][ny] = true;
                }
            }
        }
    }
    false
}

fn bfs_fire_time(grid: &Vec<Vec<i32>>, start: (usize, usize), fire_time: &mut Vec<Vec<i32>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut time = 0;
    let mut deque = VecDeque::new();
    let mut visited = vec![vec![false; n]; m];
    deque.push_back(start);
    while !deque.is_empty() {
        let mut temp = VecDeque::new();
        while let Some((x, y)) = deque.pop_front() {
            visited[x][y] = true;
            fire_time[x][y] = fire_time[x][y].min(time);

            for &(dx, dy) in DIRS.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < (m as i32) && ny >= 0 && ny < (n as i32) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if grid[nx][ny] != 0 || visited[nx][ny] {
                        continue;
                    }
                    temp.push_back((nx, ny));
                }
            }
        }
        deque = temp;
        time += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::arr::maximum_minutes::{SAFE_TIME, Solution};

    #[test]
    pub fn t1() {
        let grid = vec![
            vec![0, 2, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 2, 1, 0],
            vec![0, 2, 0, 0, 1, 2, 0],
            vec![0, 0, 2, 2, 2, 0, 2],
            vec![0, 0, 0, 0, 0, 0, 0],
        ];
        println!("{}", Solution::maximum_minutes(grid));
    }

    // [[0,0,0],[2,2,0],[1,2,0]]
    #[test]
    pub fn t2() {
        let grid = vec![
            vec![0, 0, 0],
            vec![2, 2, 0],
            vec![1, 2, 0],
        ];
        assert_eq!(Solution::maximum_minutes(grid), SAFE_TIME);
    }

    #[test]
    pub fn t3() {
        let grid = vec![
            vec![0, 2, 0, 0, 1],
            vec![0, 2, 0, 2, 2],
            vec![0, 2, 0, 0, 0],
            vec![0, 0, 2, 2, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::maximum_minutes(grid), 0);
    }
}