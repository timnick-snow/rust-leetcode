#![allow(dead_code)]
/*
213. 打家劫舍 II
提示
中等
1.4K
相关企业
你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 围成一圈 ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警 。

给定一个代表每个房屋存放金额的非负整数数组，计算你 在不触动警报装置的情况下 ，今晚能够偷窃到的最高金额。



示例 1：

输入：nums = [2,3,2]
输出：3
解释：你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
示例 2：

输入：nums = [1,2,3,1]
输出：4
解释：你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。
     偷窃到的最高金额 = 1 + 3 = 4 。
示例 3：

输入：nums = [1,2,3]
输出：3


提示：

1 <= nums.length <= 100
0 <= nums[i] <= 1000
 */
struct Solution;

/*
dp[i] 表示i间房可以偷窃的最高金额

对于i号方，可以选择偷，也可以不偷
偷: dp[i] = dp[i-2] + nums[i]
不偷: dp[i] = dp[i-1]
所以 dp[i] = max(dp[i-1],dp[i-2] + nums[i])


初始状态
dp[0] = nums[0]
dp[1] = max(nums[0],nums[1])



只和 dp[i-1],dp[i-2]有关，缩减为2个常量

此题与 198. 打家劫舍 的区别在于  第一间和最后一间不能一起偷
于是我们做两次动态规划，第一次规划排除0号房间，第二次排除最后房间
然后求两者最大值
 */
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }
        // 排除第一间房
        let ans1 = Self::dynamic_rob(&nums[1..]);
        // 排除最后的房间
        let ans2 = Self::dynamic_rob(&nums[0..n - 1]);
        // 返回最大值
        std::cmp::max(ans1, ans2)
    }
    fn dynamic_rob(nums: &[i32]) -> i32 {
        let n = nums.len();
        let mut p0 = nums[0];
        let mut p1 = std::cmp::max(nums[0], nums[1]);
        for i in 2..n {
            let temp = std::cmp::max(p1, p0 + nums[i]);
            p0 = p1;
            p1 = temp;
        }
        p1
    }
}

#[cfg(test)]
mod test {
    use crate::arr::rob2::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![1, 2, 3, 1];
        let ans = Solution::rob(nums);
        assert_eq!(ans, 4);
    }
    #[test]
    pub fn t2() {
        let nums = vec![1, 2, 3, 1];
        let ans = Solution::dynamic_rob(&nums[0..nums.len()-1]);
        assert_eq!(ans, 4);
    }
}