#![allow(dead_code)]


/*
319. 灯泡开关
中等
相关标签
相关企业
初始时有 n 个灯泡处于关闭状态。第一轮，你将会打开所有灯泡。接下来的第二轮，你将会每两个灯泡关闭第二个。

第三轮，你每三个灯泡就切换第三个灯泡的开关（即，打开变关闭，关闭变打开）。第 i 轮，你每 i 个灯泡就切换第 i 个灯泡的开关。直到第 n 轮，你只需要切换最后一个灯泡的开关。

找出并返回 n 轮后有多少个亮着的灯泡。



示例 1：



输入：n = 3
输出：1
解释：
初始时, 灯泡状态 [关闭, 关闭, 关闭].
第一轮后, 灯泡状态 [开启, 开启, 开启].
第二轮后, 灯泡状态 [开启, 关闭, 开启].
第三轮后, 灯泡状态 [开启, 关闭, 关闭].

你应该返回 1，因为只有一个灯泡还亮着。
示例 2：

输入：n = 0
输出：0
示例 3：

输入：n = 1
输出：1


提示：

0 <= n <= 109
 */

struct Solution;
/*
分析可知

第i个灯泡的状态只与i的约数个数有关，
其约数个数为偶数时，切换偶数次开关最终是关闭的。
其约数个数为奇数时，切换奇数次开关最终是开启的。

约数总是成对出现的，对于任意一个数x，如果其有约数k，那么其必然有约数 x/k
当且仅当 x = k^2 时例外

于是容易得出，任意一盏灯i，如果其不是完全平方数，其必然有偶数个约数，最终是关闭的。
所以，1~n 之间完全平方数的个数就是灯最终开启的数量，而完全平方数的个数为 sqrt(n)


 */
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64 + 0.5).sqrt() as i32
    }
}