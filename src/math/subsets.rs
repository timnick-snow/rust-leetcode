#![allow(dead_code)]
/*
78. 子集
中等
2.1K
相关企业
给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。

解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。



示例 1：

输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
示例 2：

输入：nums = [0]
输出：[[],[0]]


提示：

1 <= nums.length <= 10
-10 <= nums[i] <= 10
nums 中的所有元素 互不相同
 */
struct Solution;
/*
对于任意一个元素 都有选择/不选择 两种方式
比如对于 nums = [1,2,3]
        不选择                     选择
1        []                         [1]

2        [],[1]                     [2],[1,2]

3        [],[1],[2],[1,2]           [3],[1,3],[2,3],[1,2,3]
 */
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]];
        for x in nums.into_iter() {
            let mut temp = Vec::new();
            ans.into_iter().for_each(|mut vec| {
                temp.push(vec.clone());
                vec.push(x);
                temp.push(vec);
            });
            ans = temp;
        }
        ans
    }
}


#[cfg(test)]
mod test {
    use crate::math::subsets::Solution;

    #[test]
    pub fn t1() {
        let solution = Solution::subsets(vec![1, 2, 3]);
        println!("{:?}", solution);
    }
}

