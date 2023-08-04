#![allow(dead_code)]
/*
12. 整数转罗马数字
罗马数字包含以下七种字符： I， V， X， L，C，D 和 M。

字符          数值
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。

通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：

I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
给你一个整数，将其转为罗马数字。



示例 1:

输入: num = 3
输出: "III"
示例 2:

输入: num = 4
输出: "IV"
示例 3:

输入: num = 9
输出: "IX"
示例 4:

输入: num = 58
输出: "LVIII"
解释: L = 50, V = 5, III = 3.
示例 5:

输入: num = 1994
输出: "MCMXCIV"
解释: M = 1000, CM = 900, XC = 90, IV = 4.


提示：

1 <= num <= 3999
 */
struct Solution;

// 1,4,5,9
static ROMAN_META: [[&str; 4]; 4] = [
    ["I", "IV", "V", "IX"],
    ["X", "XL", "L", "XC"],
    ["C", "CD", "D", "CM"],
    ["M", "", "", ""],
];

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut i = 0;
        let mut ans = String::new();
        while num > 0 {
            let digit = (num % 10) as usize;
            match digit {
                0 => (),
                1..=3 => {
                    let string = ROMAN_META[i][0].to_string().repeat(digit);
                    ans.insert_str(0, &string);
                }
                4 => {
                    ans.insert_str(0, ROMAN_META[i][1]);
                }
                5..=8 => {
                    let string = ROMAN_META[i][2].to_string()
                        + &ROMAN_META[i][0].to_string().repeat(digit - 5);
                    ans.insert_str(0, &string);
                }
                9 => {
                    ans.insert_str(0, ROMAN_META[i][3]);
                }
                _ => unreachable!(),
            }
            num /= 10;
            i += 1;
        }
        ans
    }
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().fold((0, '_'), |(num, last), x| {
            match (last, x) {
                ('I', 'V') => (num + 3, x),
                ('I', 'X') => (num + 8, x),
                ('X', 'L') => (num + 30, x),
                ('X', 'C') => (num + 80, x),
                ('C', 'D') => (num + 300, x),
                ('C', 'M') => (num + 800, x),
                (_, 'I') => (num + 1, x),
                (_, 'V') => (num + 5, x),
                (_, 'X') => (num + 10, x),
                (_, 'L') => (num + 50, x),
                (_, 'C') => (num + 100, x),
                (_, 'D') => (num + 500, x),
                (_, 'M') => (num + 1000, x),
                _ => unreachable!()
            }
        }).0
    }
}

#[cfg(test)]
mod test {
    use crate::num::int_to_roman::Solution;

    fn expect(num: i32, expect: &str) {
        let roman = Solution::int_to_roman(num);
        assert_eq!(&roman, expect);
    }

    #[test]
    fn t1() {
        expect(3, "III");
    }

    #[test]
    fn t2() {
        expect(4, "IV");
    }

    #[test]
    fn t3() {
        expect(9, "IX");
    }

    #[test]
    fn t4() {
        expect(58, "LVIII");
    }

    #[test]
    fn t5() {
        expect(1994, "MCMXCIV");
    }
}