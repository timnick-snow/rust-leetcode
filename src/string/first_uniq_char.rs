#![allow(dead_code)]

/*
387. 字符串中的第一个唯一字符
简单
相关标签
相关企业
给定一个字符串 s ，找到 它的第一个不重复的字符，并返回它的索引 。如果不存在，则返回 -1 。



示例 1：

输入: s = "leetcode"
输出: 0
示例 2:

输入: s = "loveleetcode"
输出: 2
示例 3:

输入: s = "aabb"
输出: -1


提示:

1 <= s.length <= 105
s 只包含小写字母
 */
struct Solution;

/*
哈希表
 */
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut arr = [0; 26];
        s.as_bytes().iter()
            .for_each(|&x| arr[(x - b'a') as usize] += 1);

        s.as_bytes().iter()
            .position(|&x| arr[(x - b'a') as usize] == 1)
            .map(|x| x as i32)
            .unwrap_or(-1)
    }
}