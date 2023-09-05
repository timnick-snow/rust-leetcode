#![allow(dead_code)]
/*
201. 数字范围按位与
中等
470
相关企业
给你两个整数 left 和 right ，表示区间 [left, right] ，返回此区间内所有数字 按位与 的结果（包含 left 、right 端点）。



示例 1：

输入：left = 5, right = 7
输出：4
示例 2：

输入：left = 0, right = 0
输出：0
示例 3：

输入：left = 1, right = 2147483647
输出：0


提示：

0 <= left <= right <= 231 - 1
 */
struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let left = left as u64;
        let right = right as u64;
        let mut ans = 0;
        for i in 0..31 {
            // 第i位必须是1
            if left >> i & 1 == 1 {
                // 取left 0~i 位的结果
                let x = ((1 << i + 1) - 1) & left;
                if right - left + 1 <= (1 << i + 1) - x {
                    ans |= 1 << i;
                }
            }
        }
        ans
    }

    /*
    还有一个位移相关的算法叫做「Brian Kernighan 算法」，它用于清除二进制串中最右边的 1。
    Brian Kernighan 算法的关键在于我们每次对 number 和 number−1 进行按位与运算后
    number中最右边的 1 会被抹去变成 0
     */
    pub fn range_bitwise_and2(left: i32, mut right: i32) -> i32 {
        while left < right {
            right &= right - 1;
        }
        right
    }
}

#[cfg(test)]
mod test {
    use crate::num::range_bitwise_and::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::range_bitwise_and(5, 7);
        assert_eq!(ans, 4);
    }

    #[test]
    pub fn t2() {
        let ans = Solution::range_bitwise_and(0, 0);
        assert_eq!(ans, 0);
    }

    #[test]
    pub fn t3() {
        let ans = Solution::range_bitwise_and(1, 2147483647);
        assert_eq!(ans, 0);
    }

    #[test]
    pub fn t4() {
        let ans = Solution::range_bitwise_and(3, 4);
        assert_eq!(ans, 0);
    }
}