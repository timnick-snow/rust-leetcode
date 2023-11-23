#![allow(dead_code)]

struct Solution;
/*
420. 强密码检验器
困难
相关标签
相关企业
满足以下条件的密码被认为是强密码：

由至少 6 个，至多 20 个字符组成。
包含至少 一个小写 字母，至少 一个大写 字母，和至少 一个数字 。
不包含连续三个重复字符 (比如 "Baaabb0" 是弱密码, 但是 "Baaba0" 是强密码)。
给你一个字符串 password ，返回 将 password 修改到满足强密码条件需要的最少修改步数。如果 password 已经是强密码，则返回 0 。

在一步修改操作中，你可以：

插入一个字符到 password ，
从 password 中删除一个字符，或
用另一个字符来替换 password 中的某个字符。


示例 1：

输入：password = "a"
输出：5
示例 2：

输入：password = "aA1"
输出：3
示例 3：

输入：password = "1337C0d3"
输出：0


提示：

1 <= password.length <= 50
password 由字母、数字、点 '.' 或者感叹号 '!' 组成
 */
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let n = password.len();
        if n < 6 {
            // 需要插入的字符数a 缺失的种类数 3-kind
            let a = 6 - n as i32;
            let kind = kind_cnt(&password);
            return a.max(3 - kind);
        }
        if n <= 20 {
            let kind = kind_cnt(&password);
            let arr = same_check(&password);
            let update_len = arr.into_iter().map(|x| x / 3).sum::<i32>();
            return update_len.max(3 - kind);
        }
        // n > 20
        let a = n - 20;
        let kind = kind_cnt(&password);
        let mut arr = same_check(&password);

        // 将3k长度的连续相同字符删减1个字符 3k => 3k-1
        let mut x = 0;
        for i in 0..arr.len() {
            if arr[i] % 3 == 0 && x < a {
                x += 1;
                arr[i] -= 1;
            }
            if x == a {
                break;
            }
        }
        // 3k+1 => 3k-1
        for i in 0..arr.len() {
            if arr[i] % 3 == 1 && (x + 2) <= a {
                x += 2;
                arr[i] -= 2;
            }
            if x + 1 >= a {
                break;
            }
        }
        // 3k+2
        for i in 0..arr.len() {
            if x + 2 >= a {
                break;
            }
            let dc = (a - x).min(arr[i] as usize - 2) / 3;
            arr[i] -= dc as i32 * 3;
            x += dc * 3;
        }
        let update_len = arr.into_iter().map(|x| x / 3).sum::<i32>();
        // res
        // 只有更换字符才能增加种类
        let k = (3 - kind - update_len).max(0);
        a as i32 + update_len + k
    }
}

fn same_check(s: &String) -> Vec<i32> {
    let n = s.len();
    let mut cnt = 1;
    let mut ans = vec![];
    for i in 1..n {
        if s.as_bytes()[i] == s.as_bytes()[i - 1] {
            cnt += 1;
        } else {
            if cnt > 2 {
                ans.push(cnt);
            }
            cnt = 1;
        }
    }
    if cnt > 2 {
        ans.push(cnt);
    }
    ans
}

fn kind_cnt(s: &str) -> i32 {
    let mask = s.chars().fold(0_i32, |acc, x| {
        if x.is_ascii_uppercase() {
            acc | 1
        } else if x.is_ascii_lowercase() {
            acc | 2
        } else if x.is_ascii_digit() {
            acc | 4
        } else {
            acc
        }
    });
    mask.count_ones() as i32
}

#[cfg(test)]
mod test {
    use crate::string::strong_password_checker::Solution;

    #[test]
    pub fn t1() {
        let pass = "ABABABABABABABABABAB1".to_string();
        let ans = Solution::strong_password_checker(pass);
        println!("{}", ans);
    }
}