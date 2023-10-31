#![allow(dead_code)]
/*
368. 最大整除子集
中等
相关标签
相关企业
给你一个由 无重复 正整数组成的集合 nums ，请你找出并返回其中最大的整除子集 answer ，子集中每一元素对 (answer[i], answer[j]) 都应当满足：
answer[i] % answer[j] == 0 ，或
answer[j] % answer[i] == 0
如果存在多个有效解子集，返回其中任何一个均可。



示例 1：

输入：nums = [1,2,3]
输出：[1,2]
解释：[1,3] 也会被视为正确答案。
示例 2：

输入：nums = [1,2,4,8]
输出：[1,2,4,8]


提示：

1 <= nums.length <= 1000
1 <= nums[i] <= 2 * 109
nums 中的所有整数 互不相同
 */

use std::cmp::Reverse;
use std::collections::BTreeSet;

struct Solution;
/*
将结果集从小到大排序,任意一个数都是前面的数的倍数
ans[i] % a[i-1] = 0

考虑动态规划，记dp[i]表示以nums[i]结尾的最大整除子集的个数，即nums[i]必须存在于answer集合中，由于我们进行了排序，其必然是最后一个元素(最大的元素)。

转移方程: dp[i] = 1 + MAX(dp[j]) , 其中`0<=j<i` 且满足 `nums[i] % nums[j] = 0`，表示nums[i]可以追加到num[j]后面形成更大的子集

初始状态: dp[0] = 1

求出最大子集个数后，我们需要还原成实际的子集，假设dp[k]为最大值，那么nums[k]是answer的最后一个元素。
然后我们找到 dp[x] = dp[k] - 1，且满足 `nums[k] % nums[x] = 0`, 那么nums[x]是answer的倒数第二个元素，不断寻找直到dp[x]=1

为了方便取出dp中的最大值，--我们使用优先队列辅助--(不太行)，需要从大到小搜索，使用BST存储
 */
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n < 2 {
            return nums;
        }
        nums.sort_unstable();
        // let mut dp = vec![1; n];
        // 存储dp
        let mut tree_set: BTreeSet<(Reverse<i32>, usize)> = BTreeSet::new();
        tree_set.insert((Reverse(1), 0));

        // 状态方程求解dp
        for i in 1..n {
            let mut find = false;
            for &(Reverse(val), j) in tree_set.iter() {
                if nums[i] % nums[j] == 0 {
                    // dp[i] = dp[j] + 1;
                    tree_set.insert((Reverse(val + 1), i));
                    find = true;
                    break;
                }
            }
            if !find {
                tree_set.insert((Reverse(1), i));
            }
        }

        let mut iter = tree_set.into_iter();
        let (Reverse(val), mut k) = iter.next().unwrap();
        let mut ans = vec![0; val as usize];
        let mut rest = val as usize;
        loop {
            rest -= 1;
            ans[rest] = nums[k];
            if rest < 1 {
                break;
            }
            loop {
                let (Reverse(next_val), x) = iter.next().unwrap();
                if next_val == rest as i32 && nums[k] % nums[x] == 0 {
                    k = x;
                    break;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::largest_divisible_subset::Solution;

    #[test]
    pub fn t1() {
        let a = vec![1, 2, 3];
        let ans = Solution::largest_divisible_subset(a);
        println!("{:?}", ans);
    }
}