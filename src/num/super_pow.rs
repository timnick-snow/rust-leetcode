#![allow(dead_code)]
/*
372. 超级次方
中等
相关标签
相关企业
你的任务是计算 ab 对 1337 取模，a 是一个正整数，b 是一个非常大的正整数且会以数组形式给出。



示例 1：

输入：a = 2, b = [3]
输出：8
示例 2：

输入：a = 2, b = [1,0]
输出：1024
示例 3：

输入：a = 1, b = [4,3,3,8,5,2]
输出：1
示例 4：

输入：a = 2147483647, b = [2,0,0]
输出：1198


提示：

1 <= a <= 231 - 1
1 <= b.length <= 2000
0 <= b[i] <= 9
b 不含前导 0
 */
struct Solution;
/*
大数处理：a^n % m = (a%m)^n % m
可利用二项式定理证明。
a = km+x , 利用二项式定理将(km+x)^n展开，可知除最后一项x^n外，其余均可被m整除
 */

const MOD: i32 = 1337;

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        if a == 1 {
            return 1;
        }

        let base = a % MOD;
        let mut ans = 1;
        for x in b.into_iter() {
            ans = pow(ans, 10) * pow(base, x) % MOD;
        }
        ans
    }
}

fn pow(mut x: i32, mut n: i32) -> i32 {
    let mut ans = 1;
    while n != 0 {
        if n & 1 != 0 {
            ans = ans * x % MOD;
        }
        x = x * x % MOD;
        n >>= 1;
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::num::super_pow::{pow, Solution};

    #[test]
    pub fn t1() {
        let ans = Solution::super_pow(2123, vec![3, 1, 2, 4]);
        println!("{}", ans);
        println!("{}", pow(2,3));
    }
}