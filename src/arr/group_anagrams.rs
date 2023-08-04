#![allow(dead_code)]
/*
49. 字母异位词分组
中等
1.6K
相关企业
给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。

字母异位词 是由重新排列源单词的所有字母得到的一个新单词。



示例 1:

输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
示例 2:

输入: strs = [""]
输出: [[""]]
示例 3:

输入: strs = ["a"]
输出: [["a"]]


提示：

1 <= strs.length <= 104
0 <= strs[i].length <= 100
strs[i] 仅包含小写字母
 */
use std::collections::HashMap;

struct Solution;

const PRIME: isize = 100_000_007;

/*
a[0] * 31 + a[1] * 31^2 + ...
 */
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let hash = |s: &str| {
            let mut arr = [0; 26];
            s.chars().map(|x| x as u8 - b'a')
                .for_each(|x| arr[x as usize] += 1);
            let mut h = 0;
            for &x in arr.iter() {
                h = (h * 31 + x) % PRIME;
            }
            h
        };
        let mut map = HashMap::new();
        for x in strs.into_iter() {
            let h = hash(&x);
            let entry = map.entry(h).or_insert(Vec::new());
            entry.push(x);
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::arr::group_anagrams::Solution;

    #[test]
    fn t1() {
        let strs = vec![
            "eat".into(),
            "tea".into(),
            "tan".into(),
            "ate".into(),
            "nat".into(),
            "bat".into(),
        ];
        let vec2 = Solution::group_anagrams(strs);
        println!("{:?}", vec2);

    }
    #[test]
    fn t2() {
        let a = 4.0_f64;
        println!("{}", a.powi(3));
    }
}