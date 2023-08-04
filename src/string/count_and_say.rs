#![allow(dead_code)]

struct Solution;
/*
38. 外观数列
提示
中等
1K
相关企业
给定一个正整数 n ，输出外观数列的第 n 项。

「外观数列」是一个整数序列，从数字 1 开始，序列中的每一项都是对前一项的描述。

你可以将其视作是由递归公式定义的数字字符串序列：

countAndSay(1) = "1"
countAndSay(n) 是对 countAndSay(n-1) 的描述，然后转换成另一个数字字符串。
前五项如下：

1.     1
2.     11
3.     21
4.     1211
5.     111221
第一项是数字 1
描述前一项，这个数是 1 即 “ 一 个 1 ”，记作 "11"
描述前一项，这个数是 11 即 “ 二 个 1 ” ，记作 "21"
描述前一项，这个数是 21 即 “ 一 个 2 + 一 个 1 ” ，记作 "1211"
描述前一项，这个数是 1211 即 “ 一 个 1 + 一 个 2 + 二 个 1 ” ，记作 "111221"
 */


impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".into();
        }
        let last = Solution::count_and_say(n - 1);
        let mut ans = String::new();
        let mut state = (last.chars().nth(0).unwrap(), 1);
        for x in last.chars().skip(1) {
            if state.0 == x {
                state.1 += 1;
            } else {
                ans.push_str(state.1.to_string().as_str());
                ans.push(state.0);
                state = (x, 1);
            }
        }
        ans.push_str(state.1.to_string().as_str());
        ans.push(state.0);
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::string::count_and_say::Solution;

    #[test]
    fn t1() {
        let string = Solution::count_and_say(10);
        println!("{}", string);
    }
}