#![allow(dead_code)]
/*
423. 从英文中重建数字
中等
相关标签
相关企业
给你一个字符串 s ，其中包含字母顺序打乱的用英文单词表示的若干数字（0-9）。按 升序 返回原始的数字。



示例 1：

输入：s = "owoztneoer"
输出："012"
示例 2：

输入：s = "fviefuro"
输出："45"


提示：

1 <= s.length <= 105
s[i] 为 ["e","g","f","i","h","o","n","s","r","u","t","w","v","x","z"] 这些字符之一
s 保证是一个符合题目要求的字符串
 */
struct Solution;

/*
z -> zero
w -> two
u -> four
x -> six
g -> eight

o -> one
f -> five
i -> nine
r -> three
s -> seven
 */
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut chars = [0_usize; 26];
        s.as_bytes().iter().for_each(|&c| {
            chars[(c - b'a') as usize] += 1;
        });

        let mut nums = [0_usize; 10];
        help(&mut chars, 0, 'z', "zero".to_string(), &mut nums);
        help(&mut chars, 2, 'w', "two".to_string(), &mut nums);
        help(&mut chars, 4, 'u', "four".to_string(), &mut nums);
        help(&mut chars, 6, 'x', "six".to_string(), &mut nums);
        help(&mut chars, 8, 'g', "eight".to_string(), &mut nums);

        help(&mut chars, 1, 'o', "one".to_string(), &mut nums);
        help(&mut chars, 5, 'f', "five".to_string(), &mut nums);
        help(&mut chars, 9, 'i', "nine".to_string(), &mut nums);
        help(&mut chars, 3, 'r', "three".to_string(), &mut nums);
        help(&mut chars, 7, 's', "seven".to_string(), &mut nums);

        let mut ans = String::new();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                ans.push_str(&i.to_string().repeat(nums[i]));
            }
        }
        ans
    }
}

fn help(chars: &mut [usize; 26], digit: usize, ch: char, s: String, nums: &mut [usize; 10]) {
    let cnt = chars[((ch as u8) - b'a') as usize];
    nums[digit] += cnt;

    s.as_bytes().iter().for_each(|&c| {
        chars[(c - b'a') as usize] -= cnt;
    });
}