#![allow(dead_code)]
/*
1657. 确定两个字符串是否接近
中等
相关标签
相关企业
提示
如果可以使用以下操作从一个字符串得到另一个字符串，则认为两个字符串 接近 ：

操作 1：交换任意两个 现有 字符。
例如，abcde -> aecdb
操作 2：将一个 现有 字符的每次出现转换为另一个 现有 字符，并对另一个字符执行相同的操作。
例如，aacabb -> bbcbaa（所有 a 转化为 b ，而所有的 b 转换为 a ）
你可以根据需要对任意一个字符串多次使用这两种操作。

给你两个字符串，word1 和 word2 。如果 word1 和 word2 接近 ，就返回 true ；否则，返回 false 。



示例 1：

输入：word1 = "abc", word2 = "bca"
输出：true
解释：2 次操作从 word1 获得 word2 。
执行操作 1："abc" -> "acb"
执行操作 1："acb" -> "bca"
示例 2：

输入：word1 = "a", word2 = "aa"
输出：false
解释：不管执行多少次操作，都无法从 word1 得到 word2 ，反之亦然。
示例 3：

输入：word1 = "cabbba", word2 = "abbccc"
输出：true
解释：3 次操作从 word1 获得 word2 。
执行操作 1："cabbba" -> "caabbb"
执行操作 2："caabbb" -> "baaccc"
执行操作 2："baaccc" -> "abbccc"
示例 4：

输入：word1 = "cabbba", word2 = "aabbss"
输出：false
解释：不管执行多少次操作，都无法从 word1 得到 word2 ，反之亦然。


提示：

1 <= word1.length, word2.length <= 105
word1 和 word2 仅包含小写英文字母
 */
struct Solution;
/*
字符统计
 */
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let mut s1 = [0; 26];
        let mut s2 = [0; 26];
        word1.into_bytes().into_iter().for_each(|x| s1[(x - b'a') as usize] += 1);
        word2.into_bytes().into_iter().for_each(|x| s2[(x - b'a') as usize] += 1);

        for i in 0..26 {
            if (s1[i] == 0 && s2[i] != 0) || (s1[i] != 0 && s2[i] == 0) {
                return false;
            }
        }
        s1.sort_unstable();
        s2.sort_unstable();
        s1.eq(&s2)
    }
}

#[cfg(test)]
mod test {
    use crate::arr::close_strings::Solution;

    #[test]
    pub fn t1() {
        let word1 = "cabbba".to_string();
        let word2 = "abbccc".to_string();
        let ans = Solution::close_strings(word1, word2);
        println!("{}", ans);
    }
}