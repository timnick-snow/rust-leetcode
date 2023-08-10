#![allow(dead_code)]
/*
76. 最小覆盖子串
提示
困难
2.6K
相关企业
给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。



注意：

对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
如果 s 中存在这样的子串，我们保证它是唯一的答案。


示例 1：

输入：s = "ADOBECODEBANC", t = "ABC"
输出："BANC"
解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
示例 2：

输入：s = "a", t = "a"
输出："a"
解释：整个字符串 s 是最小覆盖子串。
示例 3:

输入: s = "a", t = "aa"
输出: ""
解释: t 中两个字符 'a' 均应包含在 s 的子串中，
因此没有符合条件的子字符串，返回空字符串。


提示：

m == s.length
n == t.length
1 <= m, n <= 105
s 和 t 由英文字母组成


进阶：你能设计一个在 o(m+n) 时间内解决此问题的算法吗？
 */
use std::collections::HashMap;

struct Solution;

/*
滑动窗口 left right
right扩张直至遇到目标字符 判断是否覆盖子串 然后尽可能收缩
重复上述步骤
 */
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".into();
        }
        let mut ans: Option<String> = None;
        // 统计t中各字符数量
        let mut cnt = HashMap::new();
        for x in t.into_bytes() {
            let count = cnt.entry(x).or_insert(0);
            *count += 1;
        }
        let mut map = HashMap::new();
        let vec = s.clone().into_bytes();
        // 滑动窗口指针 left right
        let mut left = 0;
        let mut right = 0;
        while right < vec.len() {
            if !cnt.contains_key(&vec[right]) {
                // 跳过不重要的字符
                right += 1;
                continue;
            }
            let count = map.entry(vec[right]).or_insert(0);
            *count += 1;
            if check(&mut map, &cnt) {
                loop {
                    // 满足条件了 尝试缩小窗口
                    while !cnt.contains_key(&vec[left]) {
                        // 跳过不重要的字符
                        left += 1;
                    }
                    // 判断是否能够删除left字符后仍然满足条件
                    let value = map.entry(vec[left]).or_default();
                    if *value > *cnt.get(&vec[left]).unwrap() {
                        // 缩小窗口 并将该字符的统计-1
                        left += 1;
                        *value -= 1;
                    } else {
                        break;
                    }
                }
                if ans.as_ref().is_none() || right - left + 1 < ans.as_ref().unwrap().len() {
                    ans = Some(s[left..=right].into());
                }
            }
            right += 1;
        }
        if ans.is_none() {
            "".into()
        } else {
            ans.unwrap()
        }
    }
}

#[inline]
fn check(map: &mut HashMap<u8, i32>, cnt: &HashMap<u8, i32>) -> bool {
    cnt.iter().all(|(&k, &v)| *map.entry(k).or_default() >= v)
}