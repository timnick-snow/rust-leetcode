#![allow(dead_code)]
/*
62. 不同路径
中等
1.8K
相关企业
一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。

问总共有多少条不同的路径？



示例 1：


输入：m = 3, n = 7
输出：28
示例 2：

输入：m = 3, n = 2
输出：3
解释：
从左上角开始，总共有 3 条路径可以到达右下角。
1. 向右 -> 向下 -> 向下
2. 向下 -> 向下 -> 向右
3. 向下 -> 向右 -> 向下
示例 3：

输入：m = 7, n = 3
输出：28
示例 4：

输入：m = 3, n = 3
输出：6


提示：

1 <= m, n <= 100
题目数据保证答案小于等于 2 * 10^9
 */
struct Solution;
/*
需要往右走 m-1 步，向下走 n-1 步
总计有 C(m-1, m+n-2)

i32容易溢出  中间计算改用u64
 */
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut ans = 1;
        let a = std::cmp::min(m - 1, n - 1) as u64;
        let mut b = (m + n - 2) as u64;
        for x in 1..=a {
            ans = ans * b / x;
            b -= 1;
        }
        ans as i32
    }
}


#[cfg(test)]
mod test {
    use crate::math::unique_paths::Solution;

    #[test]
    pub fn t1() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(51, 9), 1916797311);
    }
}

