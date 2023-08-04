/*
14. 最长公共前缀
编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ""。



示例 1：

输入：strs = ["flower","flow","flight"]
输出："fl"
示例 2：

输入：strs = ["dog","racecar","car"]
输出：""
解释：输入不存在公共前缀。


提示：

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] 仅由小写英文字母组成
*/

struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let string = strs.pop().unwrap();
        let mut n = 0;
        for (i, c) in string.chars().enumerate() {
            if strs.iter()
                .all(|x| x.chars().nth(i).map_or(false, |xc| xc == c)) {
                n = i + 1;
            } else {
                break;
            }
        }
        string[0..n].to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::string::common_prefix::Solution;

    #[test]
    fn t1() {
        let vec1 = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let string = Solution::longest_common_prefix(vec1);
        println!("{}", string);
    }
}