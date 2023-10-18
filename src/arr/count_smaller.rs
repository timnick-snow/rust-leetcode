#![allow(dead_code)]
/*
315. 计算右侧小于当前元素的个数
困难
相关标签
相关企业
给你一个整数数组 nums ，按要求返回一个新数组 counts 。数组 counts 有该性质： counts[i] 的值是  nums[i] 右侧小于 nums[i] 的元素的数量。



示例 1：

输入：nums = [5,2,6,1]
输出：[2,1,1,0]
解释：
5 的右侧有 2 个更小的元素 (2 和 1)
2 的右侧仅有 1 个更小的元素 (1)
6 的右侧有 1 个更小的元素 (1)
1 的右侧有 0 个更小的元素
示例 2：

输入：nums = [-1]
输出：[0]
示例 3：

输入：nums = [-1,-1]
输出：[0,0]


提示：

1 <= nums.length <= 105
-104 <= nums[i] <= 104
 */
use std::collections::BTreeMap;

struct Solution;

/*
使用二叉查找树处理 => 超时

二分查找 => 通过
 */
impl Solution {

    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut arr = Vec::with_capacity(n);
        for i in (0..n).rev() {
            let index = binary_search(&arr, nums[i]);
            ans[i] = index as i32;
            arr.insert(index, nums[i]);
        }
        ans
    }

    /// 超时
    pub fn count_smaller_tree(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut heap: BTreeMap<i32, i32> = BTreeMap::new();
        let mut ans = vec![0; n];
        for i in (0..n).rev() {
            // 计算之前出现的比当前数小的个数
            ans[i] = heap.range(..nums[i])
                .map(|(_, &cnt)| cnt)
                .sum::<i32>();
            // 加入到树中
            heap.entry(nums[i])
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        ans
    }
}

fn binary_search(arr: &[i32], target: i32) -> usize {
    if arr.len() == 0 {
        return 0;
    }
    let (mut left, mut right) = (0, arr.len() - 1);
    while left <= right {
        let mid = (left + right) >> 1;
        if arr[mid] >= target {
            if mid == 0 {
                return 0;
            }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    left
}

#[cfg(test)]
mod test {
    use crate::arr::count_smaller::{binary_search, Solution};

    #[test]
    pub fn t0() {
        let arr = vec![1, 6];
        assert_eq!(binary_search(&arr, 2), 1);
    }

    #[test]
    pub fn t1() {
        let arr = vec![1, 2, 2, 3, 3, 3, 6, 7, 9, 9, 9, 15];
        assert_eq!(binary_search(&arr, 0), 0);
        assert_eq!(binary_search(&arr, 2), 1);
        assert_eq!(binary_search(&arr, 3), 3);
        assert_eq!(binary_search(&arr, 8), 8);
        assert_eq!(binary_search(&arr, 20), 12);
    }


    #[test]
    pub fn t2() {
        let arr = vec![5, 2, 6, 1];
        println!("{:?}", Solution::count_smaller(arr));
    }
}