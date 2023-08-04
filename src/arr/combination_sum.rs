#![allow(dead_code)]

struct Solution;
/*
39. 组合总和
中等
2.6K
相关企业
给你一个 无重复元素 的整数数组 candidates 和一个目标整数 target ，找出 candidates 中可以使数字和为目标数 target 的 所有 不同组合 ，并以列表形式返回。你可以按 任意顺序 返回这些组合。

candidates 中的 同一个 数字可以 无限制重复被选取 。如果至少一个数字的被选数量不同，则两种组合是不同的。

对于给定的输入，保证和为 target 的不同组合数少于 150 个。



示例 1：

输入：candidates = [2,3,6,7], target = 7
输出：[[2,2,3],[7]]
解释：
2 和 3 可以形成一组候选，2 + 2 + 3 = 7 。注意 2 可以使用多次。
7 也是一个候选， 7 = 7 。
仅有这两种组合。
示例 2：

输入: candidates = [2,3,5], target = 8
输出: [[2,2,2,2],[2,3,3],[3,5]]
示例 3：

输入: candidates = [2], target = 1
输出: []


提示：

1 <= candidates.length <= 30
2 <= candidates[i] <= 40
candidates 的所有元素 互不相同
1 <= target <= 40

分析示例1
[2,3,6,7]  target=7

2,2,2,{2} !失败
2,2
2,2,3 => ANS
2,2
2,3,{3}
2,{6}
3,3,{3}
3,{6}
6,{6}
7 => ans
 */
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut ans = Vec::new();
        let mut cur = Vec::new();
        Solution::back_trace(&candidates, 0, target, &mut cur, 0, &mut ans);
        ans
    }

    fn back_trace(candidates: &[i32], start: usize, target: i32, cur: &mut Vec<i32>, cur_sum: i32, ans: &mut Vec<Vec<i32>>) {
        for i in start..candidates.len() {
            let x = candidates[i];
            if cur_sum + x == target {
                // 找到一组答案 添加到ans
                cur.push(x);
                ans.push(cur.clone());
                // 移出当前元素 退出循环
                cur.pop();
                break;
            } else if cur_sum + x < target {
                // 选择此元素
                cur.push(x);
                // 回溯
                Solution::back_trace(candidates, i, target, cur, cur_sum + x, ans);
                // 取消选择
                cur.pop();
            } else {
                break;
            }
        }
    }


    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut ans = Vec::new();
        let mut cur = Vec::new();
        Solution::dfs(&candidates, 0, target, &mut cur, &mut ans);
        ans
    }

    fn dfs(candidates: &[i32], start: usize, target: i32, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if target == 0 {
            // 找到一组答案 添加到ans
            ans.push(cur.clone());
            return;
        }
        for i in start..candidates.len() {
            let x = candidates[i];
            if x > target {
                break;
            }
            if i > start && candidates[i - 1] == x {
                continue;
            }
            cur.push(x);
            Solution::dfs(candidates, i + 1, target - x, cur, ans);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::combination_sum::Solution;

    #[test]
    fn t1() {
        let vec1 = Solution::combination_sum(vec![2, 3, 5, 6, 7, 10], 20);
        println!("{:?}", vec1);
    }
}