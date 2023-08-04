#![allow(dead_code)]
/*
33. 搜索旋转排序数组
中等
2.7K
相关企业
整数数组 nums 按升序排列，数组中的值 互不相同 。

在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。

给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。

你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。



示例 1：

输入：nums = [4,5,6,7,0,1,2], target = 0
输出：4
示例 2：

输入：nums = [4,5,6,7,0,1,2], target = 3
输出：-1
示例 3：

输入：nums = [1], target = 0
输出：-1


提示：

1 <= nums.length <= 5000
-104 <= nums[i] <= 104
nums 中的每个值都 独一无二
题目数据保证 nums 在预先未知的某个下标上进行了旋转
-104 <= target <= 104
*/
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        b_search(&nums[0..], target, 0, (nums.len() - 1) as i32)
    }
}

fn b_search(nums: &[i32], target: i32, left: i32, right: i32) -> i32 {
    // 4 5 6 7 0 1 2   target=0
    if left > right {
        -1
    } else {
        let mid = (left + right) >> 1;
        let cur = nums[mid as usize];
        if cur == target {
            return mid;
        }

        if nums[0] <= cur { // 左侧是有序的
            if cur > target {
                if nums[0] > target {
                    // 在右边搜索
                    b_search(nums, target, mid + 1, right)
                } else {
                    // 在左边搜索
                    b_search(nums, target, left, mid - 1)
                }
            } else {
                // mid < target; 那么左侧不可能满足
                // 在右边搜索
                b_search(nums, target, mid + 1, right)
            }
        } else { // 右侧是有序的
            if cur > target {
                // 右侧有序则右边不可能满足了 在左边搜索
                b_search(nums, target, left, mid - 1)
            } else {
                // mid < target
                if nums[nums.len() - 1] < target {
                    // 在左边搜索
                    b_search(nums, target, left, mid - 1)
                } else {
                    // 在右边搜索
                    b_search(nums, target, mid + 1, right)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::rotate_search::Solution;

    #[test]
    fn t1() {
        let i = Solution::search(vec![3,1], 1);
        println!("{}", i);
    }
}