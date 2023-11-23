#![allow(dead_code)]
/*
1410. HTML 实体解析器
中等
相关标签
相关企业
提示
「HTML 实体解析器」 是一种特殊的解析器，它将 HTML 代码作为输入，并用字符本身替换掉所有这些特殊的字符实体。

HTML 里这些特殊字符和它们对应的字符实体包括：

双引号：字符实体为 &quot; ，对应的字符是 " 。
单引号：字符实体为 &apos; ，对应的字符是 ' 。
与符号：字符实体为 &amp; ，对应对的字符是 & 。
大于号：字符实体为 &gt; ，对应的字符是 > 。
小于号：字符实体为 &lt; ，对应的字符是 < 。
斜线号：字符实体为 &frasl; ，对应的字符是 / 。
给你输入字符串 text ，请你实现一个 HTML 实体解析器，返回解析器解析后的结果。



示例 1：

输入：text = "&amp; is an HTML entity but &ambassador; is not."
输出："& is an HTML entity but &ambassador; is not."
解释：解析器把字符实体 &amp; 用 & 替换
示例 2：

输入：text = "and I quote: &quot;...&quot;"
输出："and I quote: \"...\""
示例 3：

输入：text = "Stay home! Practice on Leetcode :)"
输出："Stay home! Practice on Leetcode :)"
示例 4：

输入：text = "x &gt; y &amp;&amp; x &lt; y is always false"
输出："x > y && x < y is always false"
示例 5：

输入：text = "leetcode.com&frasl;problemset&frasl;all"
输出："leetcode.com/problemset/all"


提示：

1 <= text.length <= 10^5
字符串可能包含 256 个ASCII 字符中的任意字符。
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut map = HashMap::new();
        map.insert("&quot;".to_string(), '\"');
        map.insert("&apos;".to_string(), '\'');
        map.insert("&amp;".to_string(), '&');
        map.insert("&gt;".to_string(), '>');
        map.insert("&lt;".to_string(), '<');
        map.insert("&frasl;".to_string(), '/');

        let mut ans = String::new();
        let mut chars = text.chars();
        let mut flag = false;
        loop {
            if flag {
                flag = false;
            } else {
                match chars.next() {
                    None => {
                        break;
                    }
                    Some('&') => (),
                    Some(c) => {
                        ans.push(c);
                        continue;
                    }
                }
            }

            let mut seg = String::from('&');
            while let Some(ch) = chars.next() {
                match ch {
                    ';' => {
                        seg.push(ch);
                        break;
                    }
                    '&' => {
                        flag = true;
                        break;
                    }
                    _ => seg.push(ch),
                }
            }
            if map.contains_key(&seg) {
                ans.push(*map.get(&seg).unwrap());
            } else {
                ans.push_str(&seg);
            }
        }

        ans
    }
}