#![allow(dead_code)]
/*
386. 字典序排数
中等
相关标签
相关企业
给你一个整数 n ，按字典序返回范围 [1, n] 内所有整数。

你必须设计一个时间复杂度为 O(n) 且使用 O(1) 额外空间的算法。



示例 1：

输入：n = 13
输出：[1,10,11,12,13,2,3,4,5,6,7,8,9]
示例 2：

输入：n = 2
输出：[1,2]


提示：

1 <= n <= 5 * 104
 */
struct Solution;
/*
字典序
 */
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(n as usize);
        ans.push(1);
        let mut pre = 1;
        for _ in 1..n {
            if pre * 10 <= n {
                pre *= 10;
                ans.push(pre);
                continue;
            }
            if pre % 10 < 9 && pre + 1 <= n {
                pre += 1;
                ans.push(pre);
                continue;
            }
            let mut x = pre;
            loop {
                x = x / 10;
                if x % 10 != 9 {
                    break;
                }
            }
            pre = x + 1;
            ans.push(pre);
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::num::lexical_order::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::lexical_order(192);
        println!("{:?}", ans);
    }
}