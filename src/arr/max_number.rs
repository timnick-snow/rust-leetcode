#![allow(dead_code)]
/*
321. 拼接最大数
困难
相关标签
相关企业
给定长度分别为 m 和 n 的两个数组，其元素由 0-9 构成，表示两个自然数各位上的数字。现在从这两个数组中选出 k (k <= m + n) 个数字拼接成一个新的数，要求从同一个数组中取出的数字保持其在原数组中的相对顺序。

求满足该条件的最大数。结果返回一个表示该最大数的长度为 k 的数组。

说明: 请尽可能地优化你算法的时间和空间复杂度。

示例 1:

输入:
nums1 = [3, 4, 6, 5]
nums2 = [9, 1, 2, 5, 8, 3]
k = 5
输出:
[9, 8, 6, 5, 3]
示例 2:

输入:
nums1 = [6, 7]
nums2 = [6, 0, 4]
k = 5
输出:
[6, 7, 6, 0, 4]
示例 3:

输入:
nums1 = [3, 9]
nums2 = [8, 9]
k = 3
输出:
[9, 8, 9]
 */
struct Solution;

/*
单调栈

两个数组一共要挑选k个数，每个数组里面挑选的数要保持原相对顺序

第一个数组选x个数的子序列，第二个数组选y个数的子序列，x+y=k，遍历所有可能的x,y求解
 */
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let len1 = nums1.len();
        let len2 = nums2.len();

        let min_i = (k - len2 as i32).max(0) as usize;
        let max_i = len1.min(k as usize);

        let mut ans = Vec::new();
        for i in min_i..=max_i {
            let sub1 = pick_sub(&nums1, i);
            let sub2 = pick_sub(&nums2, k as usize - i);
            // println!("sub1 = {:?}", sub1);
            // println!("sub2 = {:?}", sub2);
            let merge_ans = merge(sub1, sub2);
            //println!("merge_ans = {:?}", merge_ans);
            if cmp_arr(&ans, &merge_ans) < 0 {
                ans = merge_ans;
            }
        }

        ans
    }
}

fn cmp_arr(a: &[i32], b: &[i32]) -> i32 {
    let short = a.len().min(b.len());
    for i in 0..short {
        if a[i] != b[i] {
            return a[i] - b[i];
        }
    }
    a.len() as i32 - b.len() as i32
}

fn merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let (mut p, mut q) = (0, 0);
    let mut ans = Vec::with_capacity(a.len() + b.len());
    while p < a.len() && q < b.len() {
        if cmp_arr(&a[p..], &b[q..]) >= 0 {
            ans.push(a[p]);
            p += 1;
        } else {
            ans.push(b[q]);
            q += 1;
        }
    };
    if p < a.len() {
        ans.extend_from_slice(&a[p..]);
    }
    if q < b.len() {
        ans.extend_from_slice(&b[q..]);
    }
    ans
}

fn pick_sub(arr: &[i32], k: usize) -> Vec<i32> {
    let mut remain = arr.len();
    let mut stack: Vec<i32> = Vec::new();
    for &x in arr.iter() {
        while remain > k && !stack.is_empty() && *stack.last().unwrap() < x {
            stack.pop();
            remain -= 1;
        }
        stack.push(x);
    }
    stack.iter().take(k).cloned().collect()
}

#[cfg(test)]
mod test {
    use crate::arr::max_number::Solution;

    #[test]
    pub fn t1() {
        let nums1 = vec![3, 4, 6, 5];
        let nums2 = vec![9, 1, 2, 5, 8, 3];
        let ans = Solution::max_number(nums1, nums2, 5);
        println!("{:?}", ans);
    }

    #[test]
    pub fn t2() {
        let nums1 = vec![5, 0, 2, 1, 0, 1, 0, 3, 9, 1, 2, 8, 0, 9, 8, 1, 4, 7, 3];
        let nums2 = vec![7, 6, 7, 1, 0, 1, 0, 5, 6, 0, 5, 0];
        let ans = Solution::max_number(nums1, nums2, 31);
        println!("{:?}", ans);
    }
}