#![allow(dead_code)]
/*
313. 超级丑数
中等
相关标签
相关企业
超级丑数 是一个正整数，并满足其所有质因数都出现在质数数组 primes 中。

给你一个整数 n 和一个整数数组 primes ，返回第 n 个 超级丑数 。

题目数据保证第 n 个 超级丑数 在 32-bit 带符号整数范围内。



示例 1：

输入：n = 12, primes = [2,7,13,19]
输出：32
解释：给定长度为 4 的质数数组 primes = [2,7,13,19]，前 12 个超级丑数序列为：[1,2,4,7,8,13,14,16,19,26,28,32] 。
示例 2：

输入：n = 1, primes = [2,3,5]
输出：1
解释：1 不含质因数，因此它的所有质因数都在质数数组 primes = [2,3,5] 中。

提示：

1 <= n <= 105
1 <= primes.length <= 100
2 <= primes[i] <= 1000
题目数据 保证 primes[i] 是一个质数
primes 中的所有值都 互不相同 ，且按 递增顺序 排列
 */

use std::i64;

struct Solution;

/*
基于264. 丑数 II 的思路


维护k个指针 k = primes.length
 */

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let k = primes.len();
        let mut pointer = vec![0; k];
        let mut ugly = vec![0; n];
        ugly[0] = 1;
        for i in 1..n {
            // 计算第i个丑数
            let mut candidate = i64::MAX;
            for j in 0..k {
                candidate = candidate.min(ugly[pointer[j]] as i64 * primes[j] as i64);
            }
            ugly[i] = candidate;
            // 更新指针
            for j in 0..k {
                if ugly[pointer[j]] * primes[j] as i64 == candidate {
                    pointer[j] += 1;
                }
            }
        }

        ugly[n - 1] as i32
    }
}