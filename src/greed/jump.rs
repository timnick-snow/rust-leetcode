#![allow(dead_code)]
/*
45. 跳跃游戏 II
中等
2.2K
相关企业
给定一个长度为 n 的 0 索引整数数组 nums。初始位置为 nums[0]。

每个元素 nums[i] 表示从索引 i 向前跳转的最大长度。换句话说，如果你在 nums[i] 处，你可以跳转到任意 nums[i + j] 处:

0 <= j <= nums[i]
i + j < n
返回到达 nums[n - 1] 的最小跳跃次数。生成的测试用例可以到达 nums[n - 1]。



示例 1:

输入: nums = [2,3,1,1,4]
输出: 2
解释: 跳到最后一个位置的最小跳跃数是 2。
从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
示例 2:

输入: nums = [2,3,0,1,4]
输出: 2


提示:

1 <= nums.length <= 104
0 <= nums[i] <= 1000
题目保证可以到达 nums[n-1]
 */

struct Solution;
/*
考虑贪心算法
考虑当前所有可到达点中，价值最高的位置
价值 = 当前跳跃距离 + 目标点的可跳跃距离
 */
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut jumps = 0;
        while cur < nums.len() - 1 {
            // 当前位置可以直接跳到目的地
            if cur + nums[cur] as usize >= nums.len() - 1 {
                jumps += 1;
                break;
            }
            let mut max_value = 0;
            let mut n = 0;
            for i in 1..=nums[cur] {
                // 跳跃的目标位置
                let target = cur + i as usize;
                // 计算跳跃价值
                let value = i + nums[target];
                if value > max_value {
                    max_value = value;
                    n = i;
                }
            }
            // 跳跃到价值最高的点。更新跳跃步数和当前位置
            jumps += 1;
            cur += n as usize;
        }
        jumps
    }
}

#[cfg(test)]
mod test {
    use crate::greed::jump::Solution;

    #[test]
    fn t1() {
        let jumps = Solution::jump(vec![3, 2, 1]);
        assert_eq!(jumps, 1)
    }
}