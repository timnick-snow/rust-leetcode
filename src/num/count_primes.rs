#![allow(dead_code)]
/*
204. 计数质数
提示
中等
1.1K
相关企业
给定整数 n ，返回 所有小于非负整数 n 的质数的数量 。



示例 1：

输入：n = 10
输出：4
解释：小于 10 的质数一共有 4 个, 它们是 2, 3, 5, 7 。
示例 2：

输入：n = 0
输出：0
示例 3：

输入：n = 1
输出：0


提示：

0 <= n <= 5 * 106
 */
struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut arr = vec![1; n];
        let mut ans = 0;
        for i in 2..n {
            if arr[i] == 1 {
                ans += 1;
                let mut j = i * i;
                while j < n {
                    arr[j] = 0;
                    j += i;
                }
            }
        }
        ans
    }
}