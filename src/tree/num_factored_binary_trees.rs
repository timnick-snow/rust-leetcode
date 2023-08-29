#![allow(dead_code)]
/*
823. 带因子的二叉树
中等
126
相关企业
给出一个含有不重复整数元素的数组 arr ，每个整数 arr[i] 均大于 1。

用这些整数来构建二叉树，每个整数可以使用任意次数。其中：每个非叶结点的值应等于它的两个子结点的值的乘积。

满足条件的二叉树一共有多少个？答案可能很大，返回 对 109 + 7 取余 的结果。



示例 1:

输入: arr = [2, 4]
输出: 3
解释: 可以得到这些二叉树: [2], [4], [4, 2, 2]
示例 2:

输入: arr = [2, 4, 5, 10]
输出: 7
解释: 可以得到这些二叉树: [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2].


提示：

1 <= arr.length <= 1000
2 <= arr[i] <= 10^9
arr 中的所有值 互不相同
 */
struct Solution;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;


const MASK: u64 = 1_000_000_007;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let mut map = HashMap::new();
        let mut ans = 0;
        for i in 0..arr.len() {
            let mut num = 1_u64;
            let root = arr[i];
            let mut set: HashSet<i32, RandomState> = HashSet::from_iter(arr[0..i].iter().cloned());
            for j in 0..i {
                let other = root / arr[j];
                if root % arr[j] == 0 && set.contains(&other) {
                    let x = *map.get(&arr[j]).unwrap();
                    let y = *map.get(&other).unwrap();
                    num = (num + if arr[j] == other { x * y } else { 2 * x * y }) % MASK;
                };
                set.remove(&arr[j]);
            }
            map.insert(root, num);
            ans = (ans + num) % MASK;
        }
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use crate::tree::num_factored_binary_trees::Solution;

    #[test]
    pub fn t1() {
        let ans = Solution::num_factored_binary_trees(vec![2, 4, 5, 10, 20]);
        assert_eq!(ans, 18);
    }
}