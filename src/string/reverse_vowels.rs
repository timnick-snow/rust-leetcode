#![allow(dead_code)]
/*
345. 反转字符串中的元音字母
简单
相关标签
相关企业
给你一个字符串 s ，仅反转字符串中的所有元音字母，并返回结果字符串。

元音字母包括 'a'、'e'、'i'、'o'、'u'，且可能以大小写两种形式出现不止一次。



示例 1：

输入：s = "hello"
输出："holle"
示例 2：

输入：s = "leetcode"
输出："leotcede"


提示：

1 <= s.length <= 3 * 105
s 由 可打印的 ASCII 字符组成
 */
use std::collections::HashSet;

struct Solution;


impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let n = s.len();
        let mut s = s.into_bytes();
        let (mut left, mut right) = (0, n - 1);
        // let set = get_vowels();
        let is_vowels = |ch: char| {
            "aeiouAEIOU".find(ch).is_some()
        };

        while left < right {
            while left < right && !is_vowels(s[left] as char) {
                left += 1;
            }
            while left < right && !is_vowels(s[right] as char) {
                right -= 1;
            }
            s.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        };

        String::from_utf8(s).unwrap()
    }
}

fn get_vowels() -> HashSet<u8> {
    let mut set = HashSet::new();
    set.insert(b'a');
    set.insert(b'e');
    set.insert(b'i');
    set.insert(b'o');
    set.insert(b'u');
    set.insert(b'A');
    set.insert(b'E');
    set.insert(b'I');
    set.insert(b'O');
    set.insert(b'U');
    set
}


#[cfg(test)]
mod test {
    use crate::string::reverse_vowels::Solution;

    #[test]
    pub fn t2() {
        let ans = Solution::reverse_vowels("hello".to_string());
        println!("{}", ans);
    }
}
