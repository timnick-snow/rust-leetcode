#![allow(dead_code)]
/*
412. Fizz Buzz
简单
相关标签
相关企业
给你一个整数 n ，找出从 1 到 n 各个整数的 Fizz Buzz 表示，并用字符串数组 answer（下标从 1 开始）返回结果，其中：

answer[i] == "FizzBuzz" 如果 i 同时是 3 和 5 的倍数。
answer[i] == "Fizz" 如果 i 是 3 的倍数。
answer[i] == "Buzz" 如果 i 是 5 的倍数。
answer[i] == i （以字符串形式）如果上述条件全不满足。


示例 1：

输入：n = 3
输出：["1","2","Fizz"]
示例 2：

输入：n = 5
输出：["1","2","Fizz","4","Buzz"]
示例 3：

输入：n = 15
输出：["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]


提示：

1 <= n <= 104
 */
struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans = Vec::with_capacity(n as usize);
        for x in 1..=n {
            match (x % 3 == 0, x % 5 == 0) {
                (true, true) => ans.push("FizzBuzz".into()),
                (true, false) => ans.push("Fizz".into()),
                (false, true) => ans.push("Buzz".into()),
                _ => ans.push(x.to_string()),
            }
        }
        ans
    }
}