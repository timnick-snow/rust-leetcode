#![allow(dead_code)]

/*
60. 排列序列
困难
791
相关企业
给出集合 [1,2,3,...,n]，其所有元素共有 n! 种排列。

按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：

"123"
"132"
"213"
"231"
"312"
"321"
给定 n 和 k，返回第 k 个排列。



示例 1：

输入：n = 3, k = 3
输出："213"
示例 2：

输入：n = 4, k = 9
输出："2314"
示例 3：

输入：n = 3, k = 1
输出："123"


提示：

1 <= n <= 9
1 <= k <= n!
 */
use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut deque = (1..=n).collect::<VecDeque<_>>();
        let cache = init_factorial(n - 1);
        let mut ans: Vec<i32> = Vec::new();
        let mut rest = n;
        while k > 0 {
            let value = cache.get(&(rest - 1)).cloned().unwrap();
            match (k / value, k % value) {
                (q, 0) => {
                    ans.push(deque.remove((q - 1) as usize).unwrap());
                    ans.extend(deque.into_iter().rev());
                    break;
                }
                (q, 1) => {
                    ans.push(deque.remove(q as usize).unwrap());
                    ans.extend(deque.into_iter());
                    break;
                }
                (q, r) => {
                    ans.push(deque.remove(q as usize).unwrap());
                    k = r;
                    rest -= 1;
                }
            }
        }
        ans.iter()
            .map(|&x| (x as u8 + b'0') as char).collect()
    }
}

fn init_factorial(n: i32) -> HashMap<i32, i32> {
    let mut cur = 1;
    let mut map = HashMap::new();
    map.insert(0, 1);
    for i in 1..=n {
        cur *= i;
        map.insert(i, cur);
    }
    map
}

#[cfg(test)]
mod test {
    use crate::arr::get_permutation::Solution;

    #[test]
    pub fn t1() {
        assert_eq!(&Solution::get_permutation(1, 1), "1");
        assert_eq!(&Solution::get_permutation(3, 3), "213");
        assert_eq!(&Solution::get_permutation(4, 9), "2314");
        assert_eq!(&Solution::get_permutation(5, 1), "12345");
        assert_eq!(&Solution::get_permutation(5, 38), "24153");
    }
}