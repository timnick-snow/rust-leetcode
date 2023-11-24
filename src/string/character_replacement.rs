#![allow(dead_code)]

/*
424. 替换后的最长重复字符
中等
相关标签
相关企业
给你一个字符串 s 和一个整数 k 。你可以选择字符串中的任一字符，并将其更改为任何其他大写英文字符。该操作最多可执行 k 次。

在执行上述操作后，返回 包含相同字母的最长子字符串的长度。



示例 1：

输入：s = "ABAB", k = 2
输出：4
解释：用两个'A'替换为两个'B',反之亦然。
示例 2：

输入：s = "AABABBA", k = 1
输出：4
解释：
将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
子串 "BBBB" 有最长重复字母, 答案为 4。
可能存在其他的方法来得到同样的结果。


提示：

1 <= s.length <= 105
s 仅由大写英文字母组成
0 <= k <= s.length
 */
struct Solution;

/*
滑动窗口
 */
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        if n < 2 {
            return n as i32;
        }

        let s = s.into_bytes();
        let (mut left, mut right) = (0, 0);
        let mut ans = 0;
        let mut max_cnt = 0;
        let mut cnt = [0_usize; 26];
        // [left, right)内最多替换k个字符
        while right < n {
            // 读取right字符
            cnt[(s[right] - b'A') as usize] += 1;
            max_cnt = max_cnt.max(cnt[(s[right] - b'A') as usize]);
            right += 1;

            if right - left > max_cnt + k {
                // 替换k次不足以将所有字符变为相同
                // 左边界右移
                cnt[(s[left] - b'A') as usize] -= 1;
                left += 1;
            }
            ans = ans.max(right - left);
        }

        ans as i32
    }
}