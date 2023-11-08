#![allow(dead_code)]
/*
2609. 最长平衡子字符串
简单
相关标签
相关企业
提示
给你一个仅由 0 和 1 组成的二进制字符串 s 。

如果子字符串中 所有的 0 都在 1 之前 且其中 0 的数量等于 1 的数量，则认为 s 的这个子字符串是平衡子字符串。请注意，空子字符串也视作平衡子字符串。

返回  s 中最长的平衡子字符串长度。

子字符串是字符串中的一个连续字符序列。



示例 1：

输入：s = "01000111"
输出：6
解释：最长的平衡子字符串是 "000111" ，长度为 6 。
示例 2：

输入：s = "00111"
输出：4
解释：最长的平衡子字符串是 "0011" ，长度为  4 。
示例 3：

输入：s = "111"
输出：0
解释：除了空子字符串之外不存在其他平衡子字符串，所以答案为 0 。


提示：

1 <= s.length <= 50
'0' <= s[i] <= '1'
 */
struct Solution;
/*
中心扩展
 */
impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let n = s.len();
        let mut ans = 0;
        let mut i = 0;
        while i + 1 < n {
            if s.as_bytes()[i] == b'0' && s.as_bytes()[i + 1] == b'1' {
                let (mut left, mut right) = (i, i + 1);
                let mut len = 0;
                while right < n && s.as_bytes()[left] == b'0' && s.as_bytes()[right] == b'1' {
                    len += 2;
                    if left == 0 {
                        break;
                    }
                    left -= 1;
                    right += 1;
                }
                ans = ans.max(len);
                i = right;
            } else {
                i += 1;
            }
        }
        ans
    }
}