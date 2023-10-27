#![allow(dead_code)]
/*
357. 统计各位数字都不同的数字个数
中等
相关标签
相关企业
提示
给你一个整数 n ，统计并返回各位数字都不同的数字 x 的个数，其中 0 <= x < 10n 。


示例 1：

输入：n = 2
输出：91
解释：答案应为除去 11、22、33、44、55、66、77、88、99 外，在 0 ≤ x < 100 范围内的所有数字。
示例 2：

输入：n = 0
输出：1


提示：

0 <= n <= 8
 */
struct Solution;
/*
2位数或以上时

第一位只能选择1-9的一个数   C(9,1) = 9种选择
第二位~第n位只能从剩下的9个数选择(n-1)个数  然后排列数 A(9,n-1)

总计 9 * A(9,n-1)
 */
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        if n == 1 {
            return 10;
        }
        let permutation = |mut total: i32, mut x: i32| {
            let mut res = 1;
            while x > 0 {
                res *= total;
                total -= 1;
                x -= 1;
            }
            res
        };
        let mut ans = 10;
        for i in 2..=n {
            ans += 9 * permutation(9, i - 1);
        }
        ans
    }
}