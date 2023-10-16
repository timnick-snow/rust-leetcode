#![allow(dead_code)]
/*
301. 删除无效的括号
困难
相关标签
相关企业
提示
给你一个由若干括号和字母组成的字符串 s ，删除最小数量的无效括号，使得输入的字符串有效。

返回所有可能的结果。答案可以按 任意顺序 返回。



示例 1：

输入：s = "()())()"
输出：["(())()","()()()"]
示例 2：

输入：s = "(a)())()"
输出：["(a())()","(a)()()"]
示例 3：

输入：s = ")("
输出：[""]


提示：

1 <= s.length <= 25
s 由小写英文字母以及括号 '(' 和 ')' 组成
s 中至多含 20 个括号
 */
use std::collections::HashSet;

struct Solution;

/*
解决问题的最少步骤  考虑使用BFS
 */
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut current_set = HashSet::new();
        current_set.insert(s);
        loop {
            for x in current_set.iter() {
                if is_valid(&x) {
                    ans.push(x.clone());
                }
            }
            if !ans.is_empty() {
                return ans;
            }
            // 下一个结果集
            let mut next_set = HashSet::new();
            for x in current_set.into_iter() {
                for i in 0..x.len() {
                    if i > 0 && x.as_bytes()[i] == x.as_bytes()[i - 1] {
                        continue;
                    }
                    if x.as_bytes()[i] == b'(' || x.as_bytes()[i] == b')' {
                        let mut candidate = x[0..i].to_string();
                        candidate.push_str(&x[i + 1..]);
                        next_set.insert(candidate);
                    }
                }
            }
            current_set = next_set;
        }
    }
}

fn is_valid(s: &str) -> bool {
    let mut count = 0;
    for &x in s.as_bytes() {
        if x == b'(' {
            count += 1;
        } else if x == b')' {
            count -= 1;
            if count < 0 {
                return false;
            }
        }
    }
    count == 0
}