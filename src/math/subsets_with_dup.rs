#![allow(dead_code)]
/*
90. 子集 II
中等
1.1K
相关企业
给你一个整数数组 nums ，其中可能包含重复元素，请你返回该数组所有可能的子集（幂集）。

解集 不能 包含重复的子集。返回的解集中，子集可以按 任意顺序 排列。



示例 1：

输入：nums = [1,2,2]
输出：[[],[1],[1,2],[1,2,2],[2],[2,2]]
示例 2：

输入：nums = [0]
输出：[[],[0]]


提示：

1 <= nums.length <= 10
-10 <= nums[i] <= 10
 */
struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans = Vec::new();
        dfs(0, &nums[0..], &mut Vec::new(), &mut ans);
        ans
    }
}

fn dfs(mut cur: usize, nums: &[i32], state: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if cur == nums.len() {
        ans.push(state.clone());
        return;
    }
    // 选择 cur 元素
    state.push(nums[cur]);
    // 递归
    dfs(cur + 1, nums, state, ans);
    // 撤销选择
    state.pop();
    while cur + 1 < nums.len() && nums[cur + 1] == nums[cur] {
        // 跳过重复元素
        cur += 1;
    }
    // 不选择 cur 元素 直接递归
    dfs(cur + 1, nums, state, ans);
}

