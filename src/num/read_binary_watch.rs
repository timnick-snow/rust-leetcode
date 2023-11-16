#![allow(dead_code)]
/*
401. 二进制手表
简单
相关标签
相关企业
提示
二进制手表顶部有 4 个 LED 代表 小时（0-11），底部的 6 个 LED 代表 分钟（0-59）。每个 LED 代表一个 0 或 1，最低位在右侧。

例如，下面的二进制手表读取 "4:51" 。


给你一个整数 turnedOn ，表示当前亮着的 LED 的数量，返回二进制手表可以表示的所有可能时间。你可以 按任意顺序 返回答案。

小时不会以零开头：

例如，"01:00" 是无效的时间，正确的写法应该是 "1:00" 。
分钟必须由两位数组成，可能会以零开头：

例如，"10:2" 是无效的时间，正确的写法应该是 "10:02" 。


示例 1：

输入：turnedOn = 1
输出：["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
示例 2：

输入：turnedOn = 9
输出：[]


提示：

0 <= turnedOn <= 10
 */
struct Solution;

/*
小时最多3灯
分钟最多5灯
 */
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        if turned_on > 8 {
            return vec![];
        }
        let mut ans = vec![];
        for i in 0..4 {
            if turned_on < i {
                break;
            }
            let hours = helper(i, 4, 11);
            let minutes = helper(turned_on - i, 6, 59);
            hours.iter().for_each(|&h| {
                minutes.iter().for_each(|&m| {
                    ans.push(format!("{}:{:02}", h, m));
                });
            });
        }
        ans
    }
}

fn helper(rest_on: i32, n: i32, max: i32) -> Vec<i32> {
    let mut ans = vec![];
    dfs(0, rest_on, 0, n, max, &mut ans);
    ans
}

fn dfs(cur: i32, rest_on: i32, start: i32, n: i32, max: i32, res: &mut Vec<i32>) {
    if cur > max {
        return;
    }
    if rest_on == 0 {
        res.push(cur);
        return;
    }
    if n - start < rest_on {
        return;
    }
    for i in start..n {
        // set i to 1
        dfs(cur ^ (1 << i), rest_on - 1, i + 1, n, max, res);
    }
}

#[cfg(test)]
mod test {
    use crate::num::read_binary_watch::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::read_binary_watch(1);
        println!("{:?}", ans);
    }
}
