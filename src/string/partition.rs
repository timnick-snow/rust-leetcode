#![allow(dead_code)]
/*
131. 分割回文串
中等
1.6K
相关企业
给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是 回文串 。返回 s 所有可能的分割方案。

回文串 是正着读和反着读都一样的字符串。



示例 1：

输入：s = "aab"
输出：[["a","a","b"],["aa","b"]]
示例 2：

输入：s = "a"
输出：[["a"]]


提示：

1 <= s.length <= 16
s 仅由小写英文字母组成
 */
struct Solution;

/*
方案改进
由于要大量判断子串是否为回文串，可以使用动态规划优化
记dp[i][j]表示字符串s的子串s[i..j+1]是否为回文串
那么有
    dp[i][j] = true, if j<=i
    dp[i][j] = dp[i+1][j-1] && s[i]==s[j], 其它情况

这里的状态转移方程中： i依赖i+1, 即依赖后状态，因此i应该逆向遍历

求出状态空间后，可以O(1)时间内判断子串是否为回文串了
 */
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut ans = Vec::new();
        helper(&s, &mut Vec::new(), &mut ans);
        ans
    }
    pub fn partition2(s: String) -> Vec<Vec<String>> {
        let n = s.len();
        let s = s.into_bytes();
        // 子串回文状态空间
        let mut dp = vec![vec![true; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
            }
        }
        let mut ans = Vec::new();
        dfs(&s, 0, &mut Vec::new(), &mut ans, &dp);
        ans
    }
}
/// i: 剩余字符的开始索引
fn dfs(s: &[u8], i: usize, cur: &mut Vec<String>, ans: &mut Vec<Vec<String>>, dp: &Vec<Vec<bool>>) {
    if i == s.len() {
        ans.push(cur.clone());
        return;
    }
    for j in i..s.len() {
        if dp[i][j] {
            cur.push(String::from_utf8_lossy(&s[i..j+1]).into());
            dfs(s, j + 1, cur, ans, dp);
            cur.pop();
        }
    }
}

fn helper(s: &str, cur: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
    if s.len() == 0 {
        ans.push(cur.clone());
        return;
    }
    for i in 1..=s.len() {
        if !is_palindrome(&s[0..i]) {
            continue;
        }
        // 选择一种可行的回文分隔方案
        cur.push(String::from(&s[0..i]));
        // 剩余字符继续进行分隔
        helper(&s[i..], cur, ans);
        // 取消选择
        cur.pop();
    }
}

// 判断是否为回文串
fn is_palindrome(s: &str) -> bool {
    if s.len() < 2 {
        return true;
    }
    let arr = s.as_bytes();
    let (mut l, mut r) = (0, arr.len() - 1);
    while l < r {
        if arr[l] != arr[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

#[cfg(test)]
mod test {
    use crate::string::partition::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::partition2("aab".into());
        println!("{:?}", ans);
    }


    #[test]
    pub fn t2() {
        let ans = Solution::partition("aabaccax".into());
        println!("{:?}", ans);
    }
}