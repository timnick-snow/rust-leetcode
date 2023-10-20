#![allow(dead_code)]

/*
327. 区间和的个数
困难
相关标签
相关企业
给你一个整数数组 nums 以及两个整数 lower 和 upper 。求数组中，值位于范围 [lower, upper] （包含 lower 和 upper）之内的 区间和的个数 。

区间和 S(i, j) 表示在 nums 中，位置从 i 到 j 的元素之和，包含 i 和 j (i ≤ j)。



示例 1：
输入：nums = [-2,5,-1], lower = -2, upper = 2
输出：3
解释：存在三个区间：[0,0]、[2,2] 和 [0,2] ，对应的区间和分别是：-2 、-1 、2 。
示例 2：

输入：nums = [0], lower = 0, upper = 0
输出：1


提示：

1 <= nums.length <= 105
-231 <= nums[i] <= 231 - 1
-105 <= lower <= upper <= 105
题目数据保证答案是一个 32 位 的整数
 */


/*
树状数组

考虑前缀和数组 sum

需要找到一对下标(i,j) 使得 sum[j] - sum[i] 位于 [lower, upper]
那么对每一个下标j，以j为右端点的下标对数量，就等于 sum[0..j-1]中的所有整数，其位于[sum[j]-upper,sum[j]-lower]的数量
sum[j] - sum[i] >= lower   => sum[i] <= sum[j]-lower
sum[j] - sum[i] <= upper   => sum[i] >= sum[j]-upper


注意到整数的范围可能很大，故需要利用哈希表将所有可能出现的整数，映射到连续的整数区间内。

 */
use std::collections::{BTreeSet, HashMap};

struct Solution;

#[derive(Debug)]
struct FenwickTree {
    n: usize,
    arr: Vec<i32>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            arr: vec![0; n],
        }
    }

    fn add(&mut self, mut x: usize, delta: i32) {
        while x < self.n {
            self.arr[x] += delta;
            x += Self::lowbit(x);
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut ans = 0;
        while x != 0 {
            ans += self.arr[x];
            x -= Self::lowbit(x);
        }
        ans
    }

    fn lowbit(x: usize) -> usize {
        x & (!x + 1)
    }
}

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let lower = lower as i64;
        let upper = upper as i64;
        let n = nums.len();
        // 求出前缀和
        let mut sum = vec![0_i64; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }

        // 将前缀和（以及后续需要用到的查询整数）映射到整数空间
        let mut set: BTreeSet<i64> = BTreeSet::new();
        for &x in sum.iter() {
            set.insert(x);
            set.insert(x - lower);
            set.insert(x - upper);
        }

        let mut map: HashMap<i64, usize> = HashMap::with_capacity(set.len());
        let mut idx = 0;
        for x in set.into_iter() {
            map.insert(x, idx);
            idx += 1;
        }

        // 创建树状数组
        let mut tree = FenwickTree::new(map.len() + 1);
        // 遍历前缀和数组 找出满足条件的(i,j)下标对数量
        let mut ans = 0;
        for j in 0..sum.len() {
            let left = *map.get(&(sum[j] - upper)).unwrap();
            let right = *map.get(&(sum[j] - lower)).unwrap();
            let cnt = tree.query(right + 1) - tree.query(left);
            // println!("sum[j] = {}, cnt = {}", sum[j], cnt);
            ans += cnt;
            tree.add(*map.get(&sum[j]).unwrap() + 1, 1);
            // println!("(left,right) = {:?}", (left, right));
            // println!("tree = {:?}", tree);
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::count_range_sum::Solution;

    #[test]
    pub fn t1() {
        let num = vec![-2, 5, -1];
        let ans = Solution::count_range_sum(num, -2, 2);
        println!("{}", ans);
    }
}