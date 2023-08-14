#![allow(dead_code)]
/*
91. 解码方法
中等
1.4K
相关企业
一条包含字母 A-Z 的消息通过以下映射进行了 编码 ：

'A' -> "1"
'B' -> "2"
...
'Z' -> "26"
要 解码 已编码的消息，所有数字必须基于上述映射的方法，反向映射回字母（可能有多种方法）。例如，"11106" 可以映射为：

"AAJF" ，将消息分组为 (1 1 10 6)
"KJF" ，将消息分组为 (11 10 6)
注意，消息不能分组为  (1 11 06) ，因为 "06" 不能映射为 "F" ，这是由于 "6" 和 "06" 在映射中并不等价。

给你一个只含数字的 非空 字符串 s ，请计算并返回 解码 方法的 总数 。

题目数据保证答案肯定是一个 32 位 的整数。



示例 1：

输入：s = "12"
输出：2
解释：它可以解码为 "AB"（1 2）或者 "L"（12）。
示例 2：

输入：s = "226"
输出：3
解释：它可以解码为 "BZ" (2 26), "VF" (22 6), 或者 "BBF" (2 2 6) 。
示例 3：

输入：s = "06"
输出：0
解释："06" 无法映射到 "F" ，因为存在前导零（"6" 和 "06" 并不等价）。


提示：

1 <= s.length <= 100
s 只包含数字，并且可能包含前导零。
 */
struct Solution;
/*
考虑动态规划
dp[i] 表示  截止到字符串s索引i处，解码 方法的 总数。

那么有
if s[i]=0 && s[i-1] = 1..3, dp[i]=dp[i-2]; 当前字符只能和前一个字符构成一个解码单元
if 11~19 || 21~26, dp[i] = dp[i-1] + dp[i-2]; 当前字符可以独立解码，也可以和前一个字符构成一个解码单元
其它情况,  dp[i] = dp[i-1];    当前字符只能独立解码

根据0,1索引字符确定初始状态

状态转移只与dp[i-2] dp[i-1]有关  可以简化为2个变量
pp p
 */
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let s = s.into_bytes();
        if s[0] == b'0' {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut pp = 1;
        let mut p = if (s[0] == b'1' && s[1] != b'0') || (s[0] == b'2' && s[1] > b'0' && s[1] < b'7') {
            2
        } else if s[1] == b'0' && s[0] > b'2' {
            0
        } else {
            1
        };
        for i in 2..n {
            let cur;
            if s[i] == b'0' {
                if s[i - 1] == b'1' || s[i - 1] == b'2' {
                    cur = pp;
                } else {
                    return 0;
                }
            } else if (s[i - 1] == b'1' && s[i] != b'0') || (s[i - 1] == b'2' && s[i] > b'0' && s[i] < b'7') {
                cur = pp + p;
            } else {
                cur = p;
            }
            pp = p;
            p = cur;
        }
        p
    }
}


#[cfg(test)]
mod test {
    use crate::math::num_decodings::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::num_decodings("10011".into());
        println!("{:?}", ans);
    }
}