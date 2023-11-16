#![allow(dead_code)]
/*
400. 第 N 位数字
中等
相关标签
相关企业
给你一个整数 n ，请你在无限的整数序列 [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...] 中找出并返回第 n 位上的数字。



示例 1：

输入：n = 3
输出：3
示例 2：

输入：n = 11
输出：0
解释：第 11 位数字在序列 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... 里是 0 ，它是 10 的一部分。


提示：

1 <= n <= 231 - 1
 */
struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let n = n as i64;
        // 位数
        let mut dc = 1;
        // 数字数量
        let mut nc = 9;
        // 累计位数
        let mut cur = 0;
        while n > cur + dc * nc {
            cur += dc * nc;
            dc += 1;
            nc *= 10;
        }

        let rest = n - cur;
        let mut x = rest / dc;
        let mut y = rest % dc;
        if y == 0 {
            x -= 1;
            y = dc;
        }
        y = dc - y;

        let num = 10_i32.pow(dc as u32 - 1) + x as i32;

        num / 10_i32.pow(y as u32) % 10
    }
}

#[cfg(test)]
mod test {
    use crate::num::find_nth_digit::Solution;

    #[test]
    pub fn t1() {
        let n = 1000000000;
        println!("{}", Solution::find_nth_digit(n));
    }
}