#![allow(dead_code)]
/*
342. 4的幂
简单
相关标签
相关企业
给定一个整数，写一个函数来判断它是否是 4 的幂次方。如果是，返回 true ；否则，返回 false 。

整数 n 是 4 的幂次方需满足：存在整数 x 使得 n == 4x



示例 1：

输入：n = 16
输出：true
示例 2：

输入：n = 5
输出：false
示例 3：

输入：n = 1
输出：true


提示：

-231 <= n <= 231 - 1


进阶：你能不使用循环或者递归来完成本题吗？
 */
struct Solution;

const MASK: i32 = 0x2AAA_AAAA;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && n & MASK == 0
    }
}