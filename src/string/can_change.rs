#![allow(dead_code)]
/*
2337. 移动片段得到字符串
提示
中等
49
相关企业
给你两个字符串 start 和 target ，长度均为 n 。每个字符串 仅 由字符 'L'、'R' 和 '_' 组成，其中：

字符 'L' 和 'R' 表示片段，其中片段 'L' 只有在其左侧直接存在一个 空位 时才能向 左 移动，而片段 'R' 只有在其右侧直接存在一个 空位 时才能向 右 移动。
字符 '_' 表示可以被 任意 'L' 或 'R' 片段占据的空位。
如果在移动字符串 start 中的片段任意次之后可以得到字符串 target ，返回 true ；否则，返回 false 。



示例 1：

输入：start = "_L__R__R_", target = "L______RR"
输出：true
解释：可以从字符串 start 获得 target ，需要进行下面的移动：
- 将第一个片段向左移动一步，字符串现在变为 "L___R__R_" 。
- 将最后一个片段向右移动一步，字符串现在变为 "L___R___R" 。
- 将第二个片段向右移动散步，字符串现在变为 "L______RR" 。
可以从字符串 start 得到 target ，所以返回 true 。
示例 2：

输入：start = "R_L_", target = "__LR"
输出：false
解释：字符串 start 中的 'R' 片段可以向右移动一步得到 "_RL_" 。
但是，在这一步之后，不存在可以移动的片段，所以无法从字符串 start 得到 target 。
示例 3：

输入：start = "_R", target = "R_"
输出：false
解释：字符串 start 中的片段只能向右移动，所以无法从字符串 start 得到 target 。


提示：

n == start.length == target.length
1 <= n <= 105
start 和 target 由字符 'L'、'R' 和 '_' 组成
 */
struct Solution;
/*
直接空位才能移动，这意味着 start和target的有效字符序列是完全相同的，即除去所有空位字符后，两者相同。
 */
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let (mut start, mut target) = (start.into_bytes(), target.into_bytes());
        // 添加哨兵
        start.push(0);
        target.push(0);
        let n = target.len() + 1;
        // 初始化双指针
        let (mut p, mut q) = (0, 0);
        while q < n {
            // q移动到非空位字符
            while target[q] == b'_' {
                q += 1;
            }
            // p移动到非空位字符
            while start[p] == b'_' {
                p += 1;
            }
            match (start[p], target[q]) {
                // 移动到末尾
                (0, 0) => {
                    break;
                }
                // 下一个有效字符均为 'L'
                (b'L', b'L') => {
                    // p指针的'L'字符 无法右移
                    if p < q {
                        return false;
                    }
                }
                // 下一个有效字符均为 'R'
                (b'R', b'R') => {
                    // p指针的'R'字符 无法左移
                    if p > q {
                        return false;
                    }
                }
                // 下一个有效字符不一致
                (_, _) => {
                    return false;
                }
            }
            p += 1;
            q += 1;
        }
        true
    }
}


#[cfg(test)]
mod test {
    use crate::string::can_change::Solution;

    #[test]
    fn t1() {
        // start = "_L__R__R_", target = "L______RR"
        let start = "_L__R__R_".into();
        let target = "L______RR".into();

        let ans = Solution::can_change(start, target);
        assert!(ans)
    }
}