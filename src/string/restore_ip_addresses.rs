#![allow(dead_code)]
/*
93. 复原 IP 地址
中等
1.3K
相关企业
有效 IP 地址 正好由四个整数（每个整数位于 0 到 255 之间组成，且不能含有前导 0），整数之间用 '.' 分隔。

例如："0.1.2.201" 和 "192.168.1.1" 是 有效 IP 地址，但是 "0.011.255.245"、"192.168.1.312" 和 "192.168@1.1" 是 无效 IP 地址。
给定一个只包含数字的字符串 s ，用以表示一个 IP 地址，返回所有可能的有效 IP 地址，这些地址可以通过在 s 中插入 '.' 来形成。你 不能 重新排序或删除 s 中的任何数字。你可以按 任何 顺序返回答案。



示例 1：

输入：s = "25525511135"
输出：["255.255.11.135","255.255.111.35"]
示例 2：

输入：s = "0000"
输出：["0.0.0.0"]
示例 3：

输入：s = "101023"
输出：["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]


提示：

1 <= s.length <= 20
s 仅由数字组成
 */
struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let n = s.len();
        if n < 4 || n > 15 {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let s = s.into_bytes();
        dfs(0, 0, &s, &mut String::new(), &mut ans);
        ans
    }
}

fn dfs(cur: i32, start: usize, s: &Vec<u8>, ip: &mut String, ans: &mut Vec<String>) {
    if start > s.len() - 1 {
        return;
    }
    if cur == 3 {
        let count = s.len() - start;
        let rest = match count {
            1 => String::from(s[start] as char),
            2 if s[start] != b'0' => {
                let mut temp = String::from(s[start] as char);
                temp.push(s[start + 1] as char);
                temp
            }
            3 => {
                let part = check(s[start], s[start + 1], s[start + 2]);
                if part.is_none() {
                    return;
                }
                part.unwrap()
            }
            _ => {
                return;
            }
        };
        let mut temp_ip = ip.clone();
        temp_ip.push_str(&rest);
        ans.push(temp_ip);
        return;
    }

    // 当前1个字符后面加点号
    ip.push(s[start] as char);
    ip.push('.');
    dfs(cur + 1, start + 1, s, ip, ans);
    // 撤销
    ip.drain(ip.len() - 2..);

    // 2个字符后面加点号
    if s[start] != b'0' && start + 1 < s.len() {
        ip.push(s[start] as char);
        ip.push(s[start + 1] as char);
        ip.push('.');
        dfs(cur + 1, start + 2, s, ip, ans);
        // 撤销
        ip.drain(ip.len() - 3..);
    }

    // 3个字符后面加点号
    if start + 2 < s.len() {
        let part = check(s[start], s[start + 1], s[start + 2]);
        if part.is_none() {
            return;
        }
        ip.push_str(&part.unwrap());
        ip.push('.');
        dfs(cur + 1, start + 3, s, ip, ans);
        // 撤销
        ip.drain(ip.len() - 4..);
    }
}

fn check(a: u8, b: u8, c: u8) -> Option<String> {
    let (a, b, c) = (a - b'0', b - b'0', c - b'0');
    if a == 0 {
        return None;
    }
    let num = a as i32 * 100 + b as i32 * 10 + c as i32;
    if num <= 255 {
        Some(num.to_string())
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use crate::string::restore_ip_addresses::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::restore_ip_addresses("101023".into());
        println!("{:?}", ans);
    }

    #[test]
    pub fn t2() {
        let ans = Solution::restore_ip_addresses("25525511135".into());
        println!("{:?}", ans);
    }

    #[test]
    pub fn t3() {
        let ans = Solution::restore_ip_addresses("0000".into());
        println!("{:?}", ans);
    }
}