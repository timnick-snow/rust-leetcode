#![allow(dead_code)]
/*
75. 颜色分类
提示
中等
1.6K
相关企业
给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。

我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。

必须在不使用库内置的 sort 函数的情况下解决这个问题。



示例 1：

输入：nums = [2,0,2,1,1,0]
输出：[0,0,1,1,2,2]
示例 2：

输入：nums = [2,0,1]
输出：[0,1,2]


提示：

n == nums.length
1 <= n <= 300
nums[i] 为 0、1 或 2


进阶：

你能想出一个仅使用常数空间的一趟扫描算法吗？
 */
struct Solution;

/*
定义双指针 left right
把所有的0丢前面  2丢后面
 */
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        // 开始排序
        let mut cur = 0;
        while cur <= right {
            match nums[cur] {
                0 => {
                    nums[cur] = nums[left];
                    nums[left] = 0;
                    left += 1;
                    cur += 1;
                }
                1 => {
                    cur += 1;
                }
                2 => {
                    nums[cur] = nums[right];
                    nums[right] = 2;
                    if right == 0 {
                        return;
                    }
                    right -= 1;
                }
                _ => unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::arr::sort_colors::Solution;

    #[test]
    fn t1() {
        let nums = &mut vec![1, 0, 0];
        Solution::sort_colors(nums);
        println!("{:?}", nums);
    }
}