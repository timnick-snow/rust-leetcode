#![allow(dead_code)]
/*
383. 赎金信
简单
相关标签
相关企业
给你两个字符串：ransomNote 和 magazine ，判断 ransomNote 能不能由 magazine 里面的字符构成。

如果可以，返回 true ；否则返回 false 。

magazine 中的每个字符只能在 ransomNote 中使用一次。



示例 1：

输入：ransomNote = "a", magazine = "b"
输出：false
示例 2：

输入：ransomNote = "aa", magazine = "ab"
输出：false
示例 3：

输入：ransomNote = "aa", magazine = "aab"
输出：true


提示：

1 <= ransomNote.length, magazine.length <= 105
ransomNote 和 magazine 由小写英文字母组成
 */
struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cnt = [0; 26];
        magazine.as_bytes().iter()
            .for_each(|&x| cnt[(x - b'a') as usize] += 1);
        ransom_note.as_bytes().iter()
            .for_each(|&x| cnt[(x - b'a') as usize] -= 1);
        // all >= 0
        cnt.into_iter().all(|x| x >= 0)
    }
}