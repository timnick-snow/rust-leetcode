package io.github.snow.other;
/*
352. 将数据流变为多个不相交区间
困难
相关标签
相关企业
 给你一个由非负整数 a1, a2, ..., an 组成的数据流输入，请你将到目前为止看到的数字总结为不相交的区间列表。

实现 SummaryRanges 类：

SummaryRanges() 使用一个空数据流初始化对象。
void addNum(int val) 向数据流中加入整数 val 。
int[][] getIntervals() 以不相交区间 [starti, endi] 的列表形式返回对数据流中整数的总结。


示例：

输入：
["SummaryRanges", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals"]
[[], [1], [], [3], [], [7], [], [2], [], [6], []]
输出：
[null, null, [[1, 1]], null, [[1, 1], [3, 3]], null, [[1, 1], [3, 3], [7, 7]], null, [[1, 3], [7, 7]], null, [[1, 3], [6, 7]]]

解释：
SummaryRanges summaryRanges = new SummaryRanges();
summaryRanges.addNum(1);      // arr = [1]
summaryRanges.getIntervals(); // 返回 [[1, 1]]
summaryRanges.addNum(3);      // arr = [1, 3]
summaryRanges.getIntervals(); // 返回 [[1, 1], [3, 3]]
summaryRanges.addNum(7);      // arr = [1, 3, 7]
summaryRanges.getIntervals(); // 返回 [[1, 1], [3, 3], [7, 7]]
summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
summaryRanges.getIntervals(); // 返回 [[1, 3], [7, 7]]
summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
summaryRanges.getIntervals(); // 返回 [[1, 3], [6, 7]]


提示：

0 <= val <= 104
最多调用 addNum 和 getIntervals 方法 3 * 104 次


进阶：如果存在大量合并，并且与数据流的大小相比，不相交区间的数量很小，该怎么办?
 */

import java.util.Map;
import java.util.TreeMap;

/*
添加一个数，有以下几种情况
1. 和一个区间的左侧区间合并，区间[l,r]，有 val = l-1
2. 和一个区间的右侧区间合并，区间[l,r]，有 val = r+1
3. 同时满足(1,2)，将两个区间合并成大区间
4. 被某个区间完全覆盖，l<=val<=r，不会对区间产生影响
5. 不满足以上所有，自己单独形成一个区间[val,val]

对于1，需要找到一个 l>val, 且l尽可能的小，再进一步判断 => range1
对于2，需要找到一个 l<val, 且l尽可能的大，再进一步判断 => range2
对于3，结合12判断即可
对于4，合并到2，我们改为找到一个l<=val的区间   => range2

 */

/**
 * 352. 将数据流变为多个不相交区间
 *
 * @author snow
 * @since 2023/10/26
 */
public class SummaryRanges {
    TreeMap<Integer, Integer> ranges;

    public SummaryRanges() {
        ranges = new TreeMap<>();
    }

    public void addNum(int value) {
        // 大于value  --->  大于等于value+1
        Map.Entry<Integer, Integer> range1 = ranges.ceilingEntry(value + 1);
        // 小于等于value
        Map.Entry<Integer, Integer> range2 = ranges.floorEntry(value);

        if (range2 != null && range2.getKey() <= value && range2.getValue() >= value) {
            // case4
            return;
        } else {
            boolean leftMerge = range1 != null && range1.getKey() - 1 == value;
            boolean rightMerge = range2 != null && range2.getValue() + 1 == value;
            if (leftMerge && rightMerge) {
                // case3
                int left = range2.getKey(), right = range1.getValue();
                // 移除2个旧小区间
                ranges.remove(range1.getKey());
                ranges.remove(range2.getKey());
                // 新的合并的大区间
                ranges.put(left, right);
            } else if (leftMerge) {
                // case1
                int newLeft = range1.getKey() - 1;
                // 新区间
                ranges.put(newLeft, range1.getValue());
                // 移除旧区间
                ranges.remove(range1.getKey());
            } else if (rightMerge) {
                // case2
                ranges.put(range2.getKey(), range2.getValue() + 1);
            } else {
                // case5
                ranges.put(value, value);
            }
        }
    }

    public int[][] getIntervals() {
        int[][] arr = new int[ranges.size()][2];
        int i = 0;
        for (Map.Entry<Integer, Integer> entry : ranges.entrySet()) {
            arr[i][0] = entry.getKey();
            arr[i][1] = entry.getValue();
            i++;
        }
        return arr;
    }
}
