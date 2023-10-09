#![allow(dead_code)]
/*
2578. 最小和分割
简单
相关标签
相关企业
提示
给你一个正整数 num ，请你将它分割成两个非负整数 num1 和 num2 ，满足：

num1 和 num2 直接连起来，得到 num 各数位的一个排列。
换句话说，num1 和 num2 中所有数字出现的次数之和等于 num 中所有数字出现的次数。
num1 和 num2 可以包含前导 0 。
请你返回 num1 和 num2 可以得到的和的 最小 值。

注意：

num 保证没有前导 0 。
num1 和 num2 中数位顺序可以与 num 中数位顺序不同。


示例 1：

输入：num = 4325
输出：59
解释：我们可以将 4325 分割成 num1 = 24 和 num2 = 35 ，和为 59 ，59 是最小和。
示例 2：

输入：num = 687
输出：75
解释：我们可以将 687 分割成 num1 = 68 和 num2 = 7 ，和为最优值 75 。


提示：

10 <= num <= 109
 */
struct Solution;

impl Solution {
    pub fn split_num(mut num: i32) -> i32 {
        let mut vec = Vec::new();
        while num != 0 {
            let digit = num % 10;
            if digit != 0 {
                vec.push(digit);
            }
            num /= 10;
        }
        vec.sort_unstable();
        let mut a = 0;
        let mut b = 0;
        for i in (0..vec.len()).step_by(2) {
            a = a * 10 + vec[i];
            if i + 1 < vec.len() {
                b = b * 10 + vec[i + 1];
            }
        }
        a + b
    }
}