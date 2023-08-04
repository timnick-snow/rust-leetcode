#![allow(dead_code)]

use std::cmp::min;

struct Solution;
/*
给定两个大小分别为 m 和 n 的正序（从小到大）数组nums1 和nums2。请你找出并返回这两个正序数组的 中位数 。

算法的时间复杂度应该为 O(log (m+n)) 。
示例 1：

输入：nums1 = [1,3], nums2 = [2]
输出：2.00000
解释：合并数组 = [1,2,3] ，中位数 2
示例 2：

输入：nums1 = [1,2], nums2 = [3,4]
输出：2.50000
解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5

提示：

nums1.length == m
nums2.length == n
0 <= m <= 1000
0 <= n <= 1000
1 <= m + n <= 2000
-106 <= nums1[i], nums2[i] <= 106

链接：https://leetcode.cn/problems/median-of-two-sorted-arrays

1 2 3 4 8    2 4 5 7
 */

/// 1.暴力法  合并两个数组后取出中位数
impl Solution {
    /// 2. 寻找第K小的数 使用二分法不断排除 递归
    ///
    /// 假设我们要找第 k 小数，我们可以每次循环排除掉 k/2 个数。
    ///
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 两个数组的长度
        let m = nums1.len();
        let n = nums2.len();
        let k = (m + n + 1) / 2;
        if (m + n) & 1 == 0 {
            // 总长度为偶数 1+3   2,3 | 1, 2 3 4
            // 中位数是中间两个数的平均数
            (Self::get_kth(&nums1, &nums2, k) + Self::get_kth(&nums1, &nums2, k + 1)) as f64 * 0.5
        } else {
            // 总长度为奇数 2+3   3   | 1 2, 3 4 5
            // 中位数是中间数
            Self::get_kth(&nums1, &nums2, k) as f64
        }
    }

    pub fn get_kth(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        // 保证 nums1的长度小于nums2, 这样有数组为空时 一定是nums1
        if nums1.len() > nums2.len() {
            return Self::get_kth(nums2, nums1, k);
        }
        // 递归出口1：nums1为空
        if nums1.len() == 0 {
            // 此时 nums2中第K小的数就是所求
            return nums2[k - 1];
        }
        // 递归出口2：k=1
        if k == 1 {
            // 所求为最小数
            return min(nums1[0], nums2[0]);
        }
        // 求出k/2位置索引，由于k/2可能超过数组长度，需要比较
        let i = min(nums1.len(), k / 2) - 1;
        let j = min(nums2.len(), k / 2) - 1;
        return if nums1[i] < nums2[j] {
            // num1中索引i处值小，排除0~i的所有数，进入下一次递归
            Self::get_kth(&nums1[i + 1..], nums2, k - i - 1)
        } else {
            // num2中索引j处值小，排除0~j的所有数，进入下一次递归
            Self::get_kth(nums1, &nums2[j + 1..], k - j - 1)
        };
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    fn test_assert(nums1: Vec<i32>, nums2: Vec<i32>, med: f64) {
        let ans = Solution::find_median_sorted_arrays(nums1, nums2);
        assert!((ans - med).abs() <= f64::EPSILON);
    }

    #[test]
    fn exampl1() {
        test_assert(vec![1, 3], vec![2], 2.0);
    }

    #[test]
    fn exampl2() {
        test_assert(vec![1, 3], vec![2, 4], 2.5);
    }
}