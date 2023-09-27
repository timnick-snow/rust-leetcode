#![allow(dead_code)]
/*
264. 丑数 II
中等
相关标签
相关企业
提示
给你一个整数 n ，请你找出并返回第 n 个 丑数 。

丑数 就是只包含质因数 2、3 和/或 5 的正整数。



示例 1：

输入：n = 10
输出：12
解释：[1, 2, 3, 4, 5, 6, 8, 9, 10, 12] 是由前 10 个丑数组成的序列。
示例 2：

输入：n = 1
输出：1
解释：1 通常被视为丑数。


提示：

1 <= n <= 1690
 */
struct Solution;
/*
考虑以下事实
第一个丑数是1 当前丑数序列位ugly, ugly[i]表示第i个丑数
每一个后续丑数都是由当前的丑数列表的中的值 *2, *3 或 *5 后得到

那么可以认为有3个长度无限的列表，他们分别是由原丑数列表的值 *2, *3, *5得到的
因为要从小到大生成丑数，我们维护这三个列表的头指针 p2, p3, p5

那么下一个丑数 next = min(ugly[p2]*2,ugly[p3]*3,ugly[p5]*5)
然后我们更新头指针
 */
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut ugly = vec![0; n + 1];
        ugly[1] = 1;
        let (mut p2, mut p3, mut p5) = (1, 1, 1);
        for i in 2..=n {
            let next = std::cmp::min(std::cmp::min(ugly[p2] * 2, ugly[p3] * 3), ugly[p5] * 5);
            ugly[i] = next;
            if ugly[p2] * 2 == next {
                p2 += 1;
            }
            if ugly[p3] * 3 == next {
                p3 += 1;
            }
            if ugly[p5] * 5 == next {
                p5 += 1;
            }
        }
        ugly[n]
    }
}