#![allow(dead_code)]
/*
395. 至少有 K 个重复字符的最长子串
中等
相关标签
相关企业
给你一个字符串 s 和一个整数 k ，请你找出 s 中的最长子串， 要求该子串中的每一字符出现次数都不少于 k 。返回这一子串的长度。

如果不存在这样的子字符串，则返回 0。



示例 1：

输入：s = "aaabb", k = 3
输出：3
解释：最长子串为 "aaa" ，其中 'a' 重复了 3 次。
示例 2：

输入：s = "ababbc", k = 2
输出：5
解释：最长子串为 "ababb" ，其中 'a' 重复了 2 次， 'b' 重复了 3 次。


提示：

1 <= s.length <= 104
s 仅由小写英文字母组成
1 <= k <= 105
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        if n < k {
            return 0;
        }
        let mut cnt = HashMap::new();
        s.as_bytes().iter().for_each(|&x| {
            cnt.entry(x).and_modify(|x| *x += 1).or_insert(1);
        });

        help_longest(&s, cnt, k)
    }
}

fn help_longest(s: &str, cnt: HashMap<u8, usize>, k: usize) -> i32 {
    if s.len() < k {
        return 0;
    }
    let mut left_cnt: HashMap<u8, usize> = HashMap::new();
    let mut i = 0;
    while i < s.len() && *cnt.get(&s.as_bytes()[i]).unwrap() >= k {
        left_cnt.entry(s.as_bytes()[i])
            .and_modify(|x| *x += 1)
            .or_insert(1);
        i += 1;
    }

    if i == s.len() {
        return s.len() as i32;
    }

    let right_cnt = cut_down(cnt, &left_cnt);
    let left_max = help_longest(&s[0..i], left_cnt, k);
    let right_max = help_longest(&s[i + 1..], right_cnt, k);
    left_max.max(right_max)
}

fn cut_down(mut cnt: HashMap<u8, usize>, sub_cut: &HashMap<u8, usize>) -> HashMap<u8, usize> {
    sub_cut.iter().for_each(|(&key, &value)| {
        cnt.entry(key).and_modify(|x| *x -= value);
    });
    cnt
}