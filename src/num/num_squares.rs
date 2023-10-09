#![allow(dead_code)]
/*
279. 完全平方数
中等
相关标签
相关企业
给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。

完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。



示例 1：

输入：n = 12
输出：3
解释：12 = 4 + 4 + 4
示例 2：

输入：n = 13
输出：2
解释：13 = 4 + 9

提示：

1 <= n <= 104
 */
struct Solution;

/*
动态规划
f[i] 表示最少需要多少个数的平方来表示整数 i

f[i] = 1 + min{ f(i-j^2) }


「四平方和定理」

四平方和定理证明了任意一个正整数都可以被表示为至多四个正整数的平方和。
同时四平方和定理包含了一个更强的结论：当且仅当 n ≠ 4^k*(8m+7) 时，n可以表示为至多三个正整数的平方和
当 n = 4^k*(8m+7) 时，n只能表示为四个正整数的平方和


 */
impl Solution {
    pub fn num_squares0(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0; n + 1];
        for i in 1..=n {
            let mut min_cnt = i32::MAX;
            let mut j = 1;
            while j * j <= i {
                min_cnt = std::cmp::min(min_cnt, f[i - j * j]);
                j += 1;
            };
            f[i] = 1 + min_cnt;
        }
        f[n]
    }

    pub fn num_squares(n: i32) -> i32 {
        // 平方数
        let is_perfect = |x: i32| {
            let y = (x as f64).sqrt() as i32;
            y * y == x
        };
        if is_perfect(n) {
            return 1;
        }
        // 4^k*(8m+7)
        let check4 = |mut x: i32| {
            while x % 4 == 0 {
                x /= 4;
            }
            x % 8 == 7
        };
        if check4(n) {
            return 4;
        }
        // 2个数的平方
        let mut j = 1;
        while j * j <= n {
            let y = n - j * j;
            if is_perfect(y) {
                return 2;
            }
            j += 1;
        };
        return 3;
    }
}