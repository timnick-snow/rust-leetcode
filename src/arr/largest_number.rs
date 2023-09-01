#![allow(dead_code)]

/*
179. 最大数
中等
1.2K
相关企业
给定一组非负整数 nums，重新排列每个数的顺序（每个数不可拆分）使之组成一个最大的整数。

注意：输出结果可能非常大，所以你需要返回一个字符串而不是整数。



示例 1：

输入：nums = [10,2]
输出："210"
示例 2：

输入：nums = [3,30,34,5,9]
输出："9534330"


提示：

1 <= nums.length <= 100
0 <= nums[i] <= 109
 */

struct Solution;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_unstable_by(|&a, &b| {
            let a = a as u64;
            let b = b as u64;
            // let x = a * 10_u64.pow(b.ilog10() + 1) + b;
            // let y = b * 10_u64.pow(a.ilog10() + 1) + a;
            let x = a * 10_u64.pow(log10(b) + 1) + b;
            let y = b * 10_u64.pow(log10(a) + 1) + a;
            y.cmp(&x)
        });
        if nums[0] == 0 {
            return "0".into();
        }
        let mut ans = String::new();
        for x in nums.into_iter() {
            ans.push_str(&format!("{}", x));
        }
        ans
    }
}

fn log10(mut num: u64) -> u32 {
    if num == 0 {
        return 0;
    }
    let mut ans = 0;
    while num > 9 {
        num = num / 10;
        ans += 1;
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::arr::largest_number::Solution;

    #[test]
    pub fn t1() {
        let nums = vec![3, 30, 34, 5, 9];
        println!("{}", Solution::largest_number(nums));
    }


    #[test]
    pub fn t2() {
        let nums = vec![0, 0, 0, 0];
        println!("{}", Solution::largest_number(nums));
    }
}