#![allow(dead_code)]
/*
125. 验证回文串
简单
675
相关企业
如果在将所有大写字符转换为小写字符、并移除所有非字母数字字符之后，短语正着读和反着读都一样。则可以认为该短语是一个 回文串 。

字母和数字都属于字母数字字符。

给你一个字符串 s，如果它是 回文串 ，返回 true ；否则，返回 false 。



示例 1：

输入: s = "A man, a plan, a canal: Panama"
输出：true
解释："amanaplanacanalpanama" 是回文串。
示例 2：

输入：s = "race a car"
输出：false
解释："raceacar" 不是回文串。
示例 3：

输入：s = " "
输出：true
解释：在移除非字母数字字符之后，s 是一个空字符串 "" 。
由于空字符串正着反着读都一样，所以是回文串。


提示：

1 <= s.length <= 2 * 105
s 仅由可打印的 ASCII 字符组成
 */
struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.to_ascii_lowercase().into_bytes();
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            while l <= r && !s[l].is_ascii_alphanumeric() {
                l += 1;
            }
            if l > r {
                break;
            }
            while l < r && !s[r].is_ascii_alphanumeric() {
                r -= 1;
            }
            if s[l] != s[r] {
                return false;
            }
            if r == 0 {
                break;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::string::is_palindrome::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::is_palindrome("A man, a plan, a canal: Panama".into());
        assert!(ans)
    }

    #[test]
    pub fn t2() {
        let ans = Solution::is_palindrome("race a car".into());
        assert!(!ans)
    }

    #[test]
    pub fn t3() {
        let ans = Solution::is_palindrome("  ".into());
        assert!(ans)
    }


    #[test]
    pub fn t4() {
        let ans = Solution::is_palindrome("a.".into());
        assert!(ans)
    }
}