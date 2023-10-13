#![allow(dead_code)]

/*
306. 累加数
中等
相关标签
相关企业
累加数 是一个字符串，组成它的数字可以形成累加序列。

一个有效的 累加序列 必须 至少 包含 3 个数。除了最开始的两个数以外，序列中的每个后续数字必须是它之前两个数字之和。

给你一个只包含数字 '0'-'9' 的字符串，编写一个算法来判断给定输入是否是 累加数 。如果是，返回 true ；否则，返回 false 。

说明：累加序列里的数，除数字 0 之外，不会 以 0 开头，所以不会出现 1, 2, 03 或者 1, 02, 3 的情况。



示例 1：

输入："112358"
输出：true
解释：累加序列为: 1, 1, 2, 3, 5, 8 。1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
示例 2：

输入："199100199"
输出：true
解释：累加序列为: 1, 99, 100, 199。1 + 99 = 100, 99 + 100 = 199


提示：

1 <= num.length <= 35
num 仅由数字（0 - 9）组成


进阶：你计划如何处理由过大的整数输入导致的溢出?
 */
struct Solution;
/*
暴力枚举前两个数
 */
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let n = num.len();
        // 枚举第一个数
        for i in 0..n >> 1 {
            if i > 0 && num.as_bytes()[0] == b'0' {
                break;
            }
            // 枚举第二个数
            for j in i + 1..i + 1 + ((n - i - 1) >> 1) {
                if j > i + 1 && num.as_bytes()[i + 1] == b'0' {
                    break;
                }
                if check(&num, i, j) {
                    return true;
                }
            }
        }
        false
    }
}

fn check(num: &str, a: usize, b: usize) -> bool {
    let mut x = num[0..a + 1].parse::<i64>().unwrap();
    let mut y = num[a + 1..b + 1].parse::<i64>().unwrap();
    println!("{:?}", (x, y));
    let mut flag = false;
    let mut cur = b + 1;
    while cur < num.len() {
        let sum = x + y;
        let sum_len = sum.to_string().len();
        if cur + sum_len > num.len() {
            println!("len error");
            return false;
        }
        match num[cur..cur + sum_len].parse::<i64>() {
            Ok(value) => {
                if value == sum {
                    cur += sum_len;
                    x = y;
                    y = value;
                    println!("sum right. {} = {}", value, sum);
                    flag = true;
                } else {
                    println!("sum error. {} != {}", value, sum);
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }
    flag
}

#[cfg(test)]
mod test {
    use crate::num::is_additive_number::Solution;

    #[test]
    pub fn t1() {
        let num = "112358".to_string();
        let ans = Solution::is_additive_number(num);
        assert!(ans)
    }

    #[test]
    pub fn t2() {
        let num = "199100199".to_string();
        let ans = Solution::is_additive_number(num);
        assert!(ans)
    }


    #[test]
    pub fn t3() {
        let num = "211738".to_string();
        let ans = Solution::is_additive_number(num);
        assert!(ans)
    }
}