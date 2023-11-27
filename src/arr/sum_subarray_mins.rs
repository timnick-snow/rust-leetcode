#![allow(dead_code)]
/*
907. 子数组的最小值之和
中等
相关标签
相关企业
给定一个整数数组 arr，找到 min(b) 的总和，其中 b 的范围为 arr 的每个（连续）子数组。

由于答案可能很大，因此 返回答案模 10^9 + 7 。



示例 1：

输入：arr = [3,1,2,4]
输出：17
解释：
子数组为 [3]，[1]，[2]，[4]，[3,1]，[1,2]，[2,4]，[3,1,2]，[1,2,4]，[3,1,2,4]。
最小值为 3，1，2，4，1，1，2，1，1，1，和为 17。
示例 2：

输入：arr = [11,81,94,43,3]
输出：444


提示：

1 <= arr.length <= 3 * 104
1 <= arr[i] <= 3 * 104

 */
struct Solution;
/*
单调栈
 */
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut stack: Vec<(i64, i64)> = vec![];
        let mut ans: i64 = 0;
        let mut sum: i64 = 0;
        arr.into_iter().for_each(|x| {
            let x = x as i64;
            let mut cnt = 0;
            while !stack.is_empty() && stack.last().unwrap().0 >= x {
                let (y, k) = stack.pop().unwrap();
                cnt += k;
                sum = (sum - k * y + MOD) % MOD;
            }
            stack.push((x, cnt + 1));
            sum = (sum + (cnt + 1) * x) % MOD;
            ans = (ans + sum) % MOD;
        });

        ans as i32
    }
}