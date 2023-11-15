#![allow(dead_code)]
/*
397. 整数替换
中等
相关标签
相关企业
给定一个正整数 n ，你可以做如下操作：

如果 n 是偶数，则用 n / 2替换 n 。
如果 n 是奇数，则可以用 n + 1或n - 1替换 n 。
返回 n 变为 1 所需的 最小替换次数 。



示例 1：

输入：n = 8
输出：3
解释：8 -> 4 -> 2 -> 1
示例 2：

输入：n = 7
输出：4
解释：7 -> 8 -> 4 -> 2 -> 1
或 7 -> 6 -> 3 -> 2 -> 1
示例 3：

输入：n = 4
输出：2


提示：

1 <= n <= 231 - 1
 */
use std::collections::VecDeque;

struct Solution;
/*
最少变换次数  => 尝试使用BFS

贪心+位运算

考虑末尾的二进制位

末尾是 xx0   偶数，将n减半
末尾是 x01   奇数，末尾是01，先减1再减半是更好的变换方法
末尾是 x11   奇数，末尾是11，先加1再减半是更好的变换方法  边界条件n=3应该-1
 */
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let mut n = n as i64;
        let mut ans = 0;
        while n != 1 {
            if n & 1 == 0 {
                n >>= 1;
            } else if n & 2 == 0 || n == 3 {
                // 末尾是 x01
                n -= 1;
            } else {
                // 末尾是 x11
                n += 1;
            }
            ans += 1;
        }
        ans
    }

    pub fn integer_replacement_bfs(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let n = n as i64;
        let mut ans = 0;
        let mut deque = VecDeque::new();
        deque.push_back(n);

        loop {
            let mut temp = VecDeque::new();
            while let Some(x) = deque.pop_front() {
                if x == 1 {
                    return ans;
                }
                if x & 1 == 0 {
                    temp.push_back(x >> 1);
                } else {
                    temp.push_back(x + 1);
                    temp.push_back(x - 1);
                }
            }
            ans += 1;
            deque = temp;
        }
    }
}