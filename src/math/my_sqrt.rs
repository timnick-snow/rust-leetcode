#![allow(dead_code)]

/*
69. x 的平方根
提示
简单
1.4K
相关企业
给你一个非负整数 x ，计算并返回 x 的 算术平方根 。

由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。

注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。



示例 1：

输入：x = 4
输出：2
示例 2：

输入：x = 8
输出：2
解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。


提示：

0 <= x <= 2^31 - 1
 */
struct Solution;

/*
牛顿迭代法: x[i+1] = 1/2 (x[i] + C/x[i])
 */
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let c = x as f64;
        let mut x0 = x as f64;
        loop {
            let xi = 0.5 * (x0 + c / x0);
            if (xi - x0).abs() < 1e-7 {
                break;
            }
            x0 = xi;
        }
        x0 as i32
    }
}