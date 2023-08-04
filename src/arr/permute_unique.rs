#![allow(dead_code)]

/*
47. 全排列 II
中等
1.4K
相关企业
给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。



示例 1：

输入：nums = [1,1,2]
输出：
[[1,1,2],
[1,2,1],
[2,1,1]]
示例 2：

输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]


提示：

1 <= nums.length <= 8
-10 <= nums[i] <= 10
 */
struct Solution;


/*
回溯算法
1. 选择元素
2. 递归
3. 取消选择

在选择元素的时候 为了不选到重复的元素 额外定义一个used数组来存储已经选择过的标号
由于存在重复元素 需要跳过这些重复元素

==> 在同一递归深度中 相同元素只选择一次

1. 首先对原数组进行排序
2. 对于一个元素，如果它与前一个相同，而前一个元素又未被选择，那么就是重复的
 */
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
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
        }
        if i > 0 && nums[i - 1] == x && !used[i - 1] {
            continue;
        }
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


#[cfg(test)]
mod test {
    use crate::arr::permute_unique::Solution;

    #[test]
    fn t1() {
        let vec1 = Solution::permute_unique(vec![1, 1, 1, 2]);
        println!("{:?}", vec1);
    }
}