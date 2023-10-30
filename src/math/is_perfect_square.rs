#![allow(dead_code)]

/*
367. 有效的完全平方数
简单
相关标签
相关企业
给你一个正整数 num 。如果 num 是一个完全平方数，则返回 true ，否则返回 false 。

完全平方数 是一个可以写成某个整数的平方的整数。换句话说，它可以写成某个整数和自身的乘积。

不能使用任何内置的库函数，如  sqrt 。



示例 1：

输入：num = 16
输出：true
解释：返回 true ，因为 4 * 4 = 16 且 4 是一个整数。
示例 2：

输入：num = 14
输出：false
解释：返回 false ，因为 3.742 * 3.742 = 14 但 3.742 不是一个整数。


提示：

1 <= num <= 231 - 1
 */

struct Solution;

/*
牛顿迭代法
f(x) = x^2 - a   (a>=0)
求导f'(x) = 2x

过点(x0,x0^2-a) 斜率为2x的直线方程 y - (x0^2-a) = 2x0*(x - x0)
其与x轴交于点(x0/2 + a/2x0,0)

x1 = x0/2 + a/2x0
 */
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut square = num as f64;
        while (square * square - num as f64) > 1e-6 {
            square = square / 2.0 + num as f64 / (2.0 * square);
        }
        let x = square.round() as i32;
        x * x == num
    }
}