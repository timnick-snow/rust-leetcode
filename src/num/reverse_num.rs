struct Solution;
/*
给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。

如果反转后整数超过 32 位的有符号整数的范围[−2^31, 2^31− 1] ，就返回 0。

假设环境不允许存储 64 位整数（有符号或无符号）。

示例 1：

输入：x = 123
输出：321
示例 2：

输入：x = -123
输出：-321
示例 3：

输入：x = 120
输出：21
示例 4：

输入：x = 0
输出：0

提示：
-2^31 <= x <= 2^31 - 1

链接：https://leetcode.cn/problems/reverse-integer
*/
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev = 0;
        while x != 0 {
            if rev < i32::MIN / 10 || rev > i32::MAX / 10 {
                return 0;
            }
            let digit = x % 10;
            x = x / 10;
            rev = rev * 10 + digit;
        }
        rev
    }
}

#[cfg(test)]
mod test {
    use crate::num::reverse_num::Solution;

    #[test]
    fn test_three_digit() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test_large() {
        let i = Solution::reverse(-2147483412);
        println!("{}", i);
    }
}