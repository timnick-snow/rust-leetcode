#![allow(dead_code)]
/*
172. 阶乘后的零
中等
786
相关企业
给定一个整数 n ，返回 n! 结果中尾随零的数量。

提示 n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1



示例 1：

输入：n = 3
输出：0
解释：3! = 6 ，不含尾随 0
示例 2：

输入：n = 5
输出：1
解释：5! = 120 ，有一个尾随 0
示例 3：

输入：n = 0
输出：0


提示：

0 <= n <= 104


进阶：你可以设计并实现对数时间复杂度的算法来解决此问题吗？
 */
struct Solution;


impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut ans = 0;
        for x in 1..=n {
            let mut y = x;
            while y > 0 && y % 5 == 0 {
                ans += 1;
                y = y / 5;
            }
        }
        ans
    }
}