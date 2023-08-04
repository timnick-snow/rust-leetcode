use std::collections::HashMap;

struct Solution;

/*
给定一个整数数组 nums和一个整数目标值 target，请你在该数组中找出 和为目标值 target 的那两个整数，并返回它们的数组下标。

你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。

你可以按任意顺序返回答案。

输入：nums = [2,7,11,15], target = 9
输出：[0,1]
解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。

提示：

2 <= nums.length <= 104
-109 <= nums[i] <= 109
-109 <= target <= 109
只会存在一个有效答案
 */
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, &value) in nums.iter().enumerate() {
            if let Some(&i2) = map.get(&value) {
                return vec![i2, i as i32];
            } else {
                map.insert(target - value, i as i32);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_sum(nums: Vec<i32>, target: i32, res: &[i32]) {
        assert_eq!(Solution::two_sum(nums, target), res);
    }

    #[test]
    fn example1() {
        assert_sum(vec![2, 7, 11, 15], 9, &vec![0, 1]);
    }

    #[test]
    fn example2() {
        assert_sum(vec![3, 2, 4], 6, &vec![1, 2]);
    }

    #[test]
    fn example3() {
        assert_sum(vec![3, 3], 6, &vec![0, 1]);
    }
}