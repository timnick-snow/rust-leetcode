#![allow(dead_code)]
/*
174. 地下城游戏
困难
767
相关企业
恶魔们抓住了公主并将她关在了地下城 dungeon 的 右下角 。地下城是由 m x n 个房间组成的二维网格。我们英勇的骑士最初被安置在 左上角 的房间里，他必须穿过地下城并通过对抗恶魔来拯救公主。

骑士的初始健康点数为一个正整数。如果他的健康点数在某一时刻降至 0 或以下，他会立即死亡。

有些房间由恶魔守卫，因此骑士在进入这些房间时会失去健康点数（若房间里的值为负整数，则表示骑士将损失健康点数）；其他房间要么是空的（房间里的值为 0），要么包含增加骑士健康点数的魔法球（若房间里的值为正整数，则表示骑士将增加健康点数）。

为了尽快解救公主，骑士决定每次只 向右 或 向下 移动一步。

返回确保骑士能够拯救到公主所需的最低初始健康点数。

注意：任何房间都可能对骑士的健康点数造成威胁，也可能增加骑士的健康点数，包括骑士进入的左上角房间以及公主被监禁的右下角房间。



示例 1：


输入：dungeon = [[-2,-3,3],[-5,-10,1],[10,30,-5]]
输出：7
解释：如果骑士遵循最佳路径：右 -> 右 -> 下 -> 下 ，则骑士的初始健康点数至少为 7 。
示例 2：

输入：dungeon = [[0]]
输出：1


提示：

m == dungeon.length
n == dungeon[i].length
1 <= m, n <= 200
-1000 <= dungeon[i][j] <= 1000
 */
struct Solution;

/*
考虑动态规划
从前往后计算的话存在一个问题，
参数1：从(0,0)到达(i,j)点所需的最小点数x
参数2：从(0,0)到达(i,j)点的最大路径和y

最小点数x直接影响结果，这个值我们希望越小越好。
最大路径和y也会影响后续选择，这个值是战未来的，这个值我们希望越大越好。
x,y都对后续规划有影响，且这两个值可能是由不同的路径形成的。这使得我们不方便状态转移的计算


考虑从后往前进行计算，dp[i][j]表示从(i,j)点达到终点所需的最小点数x

状态转移，考虑其后续前进点，其只与dp[i+1][j], dp[i][j+1]有关
我们选择约束更少的路线，即取min(dp[i+1][j], dp[i][j+1])

那么 dp[i][j] + dungeon[i][j] = min(dp[i+1][j], dp[i][j+1])
即 dp[i][j] = min(dp[i+1][j], dp[i][j+1]) - dungeon[i][j]
需要注意，骑士在任何时候都不能死亡，其健康点数至少为 1

边界
dp[m-1][n-1] 依赖于 dp[m][n-1] 和 dp[m-1][n]，将其设置为1
 */
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
        dp[m][n - 1] = 1;
        dp[m - 1][n] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let min = std::cmp::min(dp[i + 1][j], dp[i][j + 1]);
                dp[i][j] = std::cmp::max(1, min - dungeon[i][j]);
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod test {
    use crate::dynamic::calculate_minimum_hp::Solution;

    #[test]
    pub fn t1() {
        let dungeon = vec![vec![0,0]];
        let ans = Solution::calculate_minimum_hp(dungeon);
        println!("{}", ans);
    }
}