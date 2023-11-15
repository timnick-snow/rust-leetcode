#![allow(dead_code)]
/*
394. 字符串解码
中等
相关标签
相关企业
给定一个经过编码的字符串，返回它解码后的字符串。

编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。

你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。

此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。



示例 1：

输入：s = "3[a]2[bc]"
输出："aaabcbc"
示例 2：

输入：s = "3[a2[c]]"
输出："accaccacc"
示例 3：

输入：s = "2[abc]3[cd]ef"
输出："abcabccdcdcdef"
示例 4：

输入：s = "abc3[cd]xyz"
输出："abccdcdcdxyz"


提示：

1 <= s.length <= 30
s 由小写英文字母、数字和方括号 '[]' 组成
s 保证是一个 有效 的输入。
s 中所有整数的取值范围为 [1, 300]
 */
struct Solution;
/*
本题中可能出现括号嵌套的情况，比如 2[a2[bc]]，这种情况下我们可以先转化成 2[abcbc]，在转化成 abcbcabcbc。
我们可以把字母、数字和括号看成是独立的 TOKEN，并用栈来维护这些 TOKEN。具体的做法是，遍历这个栈：

如果当前的字符为数位，解析出一个数字（连续的多个数位）并进栈

如果当前的字符为字母或者左括号，直接进栈

如果当前的字符为右括号，开始出栈，一直到左括号出栈，出栈序列反转后拼接成一个字符串，此时取出栈顶的数字，
就是这个字符串应该出现的次数，我们根据这个次数和字符串构造出新的字符串并进栈
 */
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut deque = Vec::new();
        let n = s.len();
        let s = s.into_bytes();
        let mut i = 0;
        while i < n {
            let ch = s[i] as char;
            if ch.is_ascii_digit() {
                // 取出一个数字
                let mut num = String::new();
                while (s[i] as char).is_ascii_digit() {
                    num.push(s[i] as char);
                    i += 1;
                }
                deque.push(num);
            } else if ch == '[' || ch.is_ascii_alphabetic() {
                deque.push(ch.to_string());
                i += 1;
            } else {
                // ']'
                i += 1;
                // 出栈直到遇到左括号
                let mut segment: Vec<String> = Vec::new();
                while !deque.last().unwrap().eq("[") {
                    segment.push(deque.pop().unwrap());
                }
                segment.reverse();
                // 弹出左括号
                deque.pop();
                // 取出重复次数
                let cnt = deque.pop().unwrap().parse::<usize>().unwrap();

                let s = segment.into_iter()
                    .fold(String::new(), |mut acc, x| {
                        acc.push_str(&x);
                        acc
                    });
                // 将构造好的字符串入栈
                deque.push(s.repeat(cnt));
            }
        }

        let mut ans = String::new();
        deque.into_iter().for_each(|x| ans.push_str(&x));
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::string::decode_string::Solution;

    #[test]
    pub fn t1() {
        let s = "3[a]2[bc]";
        println!("{}", Solution::decode_string(s.to_string()));
    }

    #[test]
    pub fn t2() {
        let s = "3[a2[c]]";
        println!("{}", Solution::decode_string(s.to_string()));
    }

    #[test]
    pub fn t3() {
        let s = "2[abc]3[cd]ef";
        println!("{}", Solution::decode_string(s.to_string()));
    }

    #[test]
    pub fn t4() {
        let s = "3[z]2[2[y]pq4[2[jk]e1[f]]]ef";
        println!("{}", Solution::decode_string(s.to_string()));
    }
}