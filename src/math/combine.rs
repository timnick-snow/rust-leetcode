#![allow(dead_code)]
/*
77. 组合
中等
1.5K
相关企业
给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。

你可以按 任何顺序 返回答案。



示例 1：

输入：n = 4, k = 2
输出：
[
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]
示例 2：

输入：n = 1, k = 1
输出：[[1]]


提示：

1 <= n <= 20
1 <= k <= n
 */
struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        back_trace(n as usize, k as usize, &mut Vec::new(), &mut ans, 1);
        ans
    }
}

fn back_trace(n: usize, k: usize, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, start: usize) {
    // 当cur元素数量等于k时 找到了一组解
    if cur.len() == k {
        ans.push(cur.clone());
        return;
    }
    // 依次尝试元素
    for i in start..=n {
        if n + 1 + cur.len() < k + start {
            // 剩余元素不足
            break;
        }
        // 选择
        cur.push(i as i32);
        // 回溯
        back_trace(n, k, cur, ans, i + 1);
        // 撤销选择
        cur.pop();
    }
}


#[cfg(test)]
mod test {
    use crate::math::combine::Solution;

    #[test]
    fn t1() {
        let vec = Solution::combine(5, 4);
        println!("{:?}", vec);
    }
}