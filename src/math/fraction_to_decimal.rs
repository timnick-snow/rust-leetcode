#![allow(dead_code)]
/*
166. 分数到小数
提示
中等
471
相关企业
给定两个整数，分别表示分数的分子 numerator 和分母 denominator，以 字符串形式返回小数 。

如果小数部分为循环小数，则将循环的部分括在括号内。

如果存在多个答案，只需返回 任意一个 。

对于所有给定的输入，保证 答案字符串的长度小于 104 。



示例 1：

输入：numerator = 1, denominator = 2
输出："0.5"
示例 2：

输入：numerator = 2, denominator = 1
输出："2"
示例 3：

输入：numerator = 4, denominator = 333
输出："0.(012)"


提示：

-231 <= numerator, denominator <= 231 - 1
denominator != 0
 */
use std::collections::HashMap;

struct Solution;

/*
模拟长除法
 */
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut numerator = numerator as i64;
        let mut denominator = denominator as i64;

        let mut ans = String::new();
        if (numerator > 0 && denominator < 0) || (numerator < 0 && denominator > 0) {
            ans.push('-');
        }
        numerator = numerator.abs();
        denominator = denominator.abs();
        // 整数部分
        ans.push_str(&format!("{}", numerator / denominator));
        let mut remainder = (numerator % denominator) * 10;
        if remainder == 0 {
            return ans;
        }
        // 小数点
        ans.push('.');
        // 小数部分
        let mut map = HashMap::new();
        while remainder != 0 {
            // 商
            let x = remainder / denominator;
            // 缓存 key: 余数  val: 商索引起始位置
            map.insert(remainder, ans.len());
            ans.push_str(&format!("{}", x));
            // 更新余数
            remainder = (remainder % denominator) * 10;
            if map.contains_key(&remainder) {
                ans.insert(*map.get(&remainder).unwrap(), '(');
                ans.push(')');
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::math::fraction_to_decimal::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::fraction_to_decimal(1, 2);
        assert_eq!(&ans, "0.5");
    }

    #[test]
    pub fn t2() {
        let ans = Solution::fraction_to_decimal(2, 1);
        assert_eq!(&ans, "2");
    }

    #[test]
    pub fn t3() {
        let ans = Solution::fraction_to_decimal(4, 333);
        assert_eq!(&ans, "0.(012)");
    }

    #[test]
    pub fn t4() {
        let ans = Solution::fraction_to_decimal(7, 296);
        assert_eq!(&ans, "0.023(648)");
    }
}