#![allow(dead_code)]

/*
316. 去除重复字母
中等
相关标签
相关企业
提示
给你一个字符串 s ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 返回结果的字典序最小（要求不能打乱其他字符的相对位置）。



示例 1：

输入：s = "bcabc"
输出："abc"
示例 2：

输入：s = "cbacdcbc"
输出："acdb"


提示：

1 <= s.length <= 104
s 由小写英文字母组成


注意：该题与 1081 https://leetcode-cn.com/problems/smallest-subsequence-of-distinct-characters 相同
 */
struct Solution;

/*
先统计数量 然后处理
 */
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut cnt = [0; 26];
        // 统计数量
        for &x in s.as_bytes() {
            cnt[(x - b'a') as usize] += 1;
        }
        let mut ans = String::new();
        let mut start = 0;
        while start < s.len() {
            let cur_char = s.as_bytes()[start];
            let cur_cnt = cnt[(cur_char - b'a') as usize];
            if cur_cnt == 1 {
                // 必须保留这个字符
                cnt[(cur_char - b'a') as usize] = 0;
                ans.push(cur_char as char);
                //println!("{}  => keep@1 {}", start, cur_char as char);
            } else if cur_cnt > 1 {
                // 可以移出 也可以保留
                let mut cnt_clone = cnt.clone();
                cnt_clone[(cur_char - b'a') as usize] -= 1;
                let mut end = start + 1;
                let mut flag = true; // 默认保留
                while end < s.len() {
                    let end_char = s.as_bytes()[end];
                    let end_cnt = cnt_clone[(end_char - b'a') as usize];
                    if end_cnt == 1 {
                        // end字符必然保留 这时候可以确定start是保留还是去除
                        if cur_char > end_char {
                            flag = false;
                        }
                        break;
                    } else if end_cnt > 1 {
                        if cur_char > end_char {
                            flag = false;
                            break;
                        }
                        cnt_clone[(end_char - b'a') as usize] -= 1;
                    }
                    end += 1;
                }
                if flag {
                    cnt[(cur_char - b'a') as usize] = 0;
                    ans.push(s.as_bytes()[start] as char);
                    //println!("{}  => keep@2 {}", start, s.as_bytes()[start] as char);
                } else {
                    cnt[(cur_char - b'a') as usize] -= 1;
                    //println!("{}  => skip {}", start, s.as_bytes()[start] as char);
                }
            }
            start += 1;
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::string::remove_duplicate_letters::Solution;

    #[test]
    pub fn t1() {
        let s = "bccab".to_string();
        let ans = Solution::remove_duplicate_letters(s);
        assert_eq!(&ans, "bca");
    }

    #[test]
    pub fn t2() {
        let arr = [0, 1, 2, 3];
        let mut arr2 = arr.clone();
        arr2[0] = 3;
        println!("{:?}", arr);
        println!("{:?}", arr2);
    }
}