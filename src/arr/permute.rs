#![allow(dead_code)]
/*
46. 全排列
中等
2.6K
相关企业
给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。



示例 1：

输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
示例 2：

输入：nums = [0,1]
输出：[[0,1],[1,0]]
示例 3：

输入：nums = [1]
输出：[[1]]


提示：

1 <= nums.length <= 6
-10 <= nums[i] <= 10
nums 中的所有整数 互不相同
 */
struct Solution;


/*
回溯算法
1. 选择元素
2. 递归
3. 取消选择

在选择元素的时候 为了不选到重复的元素 额外定义一个used数组来存储已经选择过的标号
 */
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut used = vec![false; nums.len()];
        dfs(&mut Vec::new(), &nums, &mut ans, &mut used);
        ans
    }
}

fn dfs(cur: &mut Vec<i32>, nums: &[i32], ans: &mut Vec<Vec<i32>>, used: &mut Vec<bool>) {
    if cur.len() == nums.len() {
        ans.push(cur.clone());
        return;
    }
    for (i, &x) in nums.iter().enumerate() {
        if used[i] {
            continue;
        } else {
            // 选择
            cur.push(x);
            used[i] = true;
            // 递归
            dfs(cur, nums, ans, used);
            // 取消选择
            cur.pop();
            used[i] = false;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::permute::Solution;

    #[test]
    fn t1() {
        let vec1 = Solution::permute(vec![1, 2, 3,4]);
        println!("{:?}", vec1);
    }
}