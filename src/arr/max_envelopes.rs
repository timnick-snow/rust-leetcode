#![allow(dead_code)]
/*
354. 俄罗斯套娃信封问题
困难
相关标签
相关企业
给你一个二维整数数组 envelopes ，其中 envelopes[i] = [wi, hi] ，表示第 i 个信封的宽度和高度。

当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。

请计算 最多能有多少个 信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。

注意：不允许旋转信封。


示例 1：

输入：envelopes = [[5,4],[6,4],[6,7],[2,3]]
输出：3
解释：最多信封的个数为 3, 组合为: [2,3] => [5,4] => [6,7]。
示例 2：

输入：envelopes = [[1,1],[1,1],[1,1]]
输出：1


提示：

1 <= envelopes.length <= 105
envelopes[i].length == 2
1 <= wi, hi <= 105
 */
use std::cmp::Ordering;
use std::collections::BTreeSet;

struct Solution;

#[derive(Debug)]
struct Envelope {
    w: i32,
    h: i32,
    cnt: i32,
}

impl Envelope {
    pub fn new(w: i32, h: i32, cnt: i32) -> Self {
        Envelope {
            w,
            h,
            cnt,
        }
    }
    pub fn area(&self) -> i64 {
        self.w as i64 * self.h as i64
    }
}

impl Eq for Envelope {}

impl PartialEq<Self> for Envelope {
    fn eq(&self, other: &Self) -> bool {
        self.cnt == other.cnt
            && self.w == other.w
            && self.h == other.h
    }
}

impl PartialOrd<Self> for Envelope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = match self.cnt.cmp(&other.cnt) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => {
                match self.area().cmp(&other.area()) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => {
                        if self.eq(other) {
                            Ordering::Equal
                        } else {
                            self.w.cmp(&other.w)
                        }
                    }
                }
            }
        };
        Some(ordering)
    }
}

impl Ord for Envelope {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let n = envelopes.len();
        envelopes.sort_unstable_by(|x, y| {
            (x[0] as i64 * x[1] as i64).cmp(&(y[0] as i64 * y[1] as i64))
        });
        // println!("{:?}", envelopes);

        let mut tree_set: BTreeSet<Envelope> = BTreeSet::new();
        let mut ans = 1;
        for i in 0..n {
            let mut flag = false;
            for x in tree_set.iter() {
                if envelopes[i][0] > x.w && envelopes[i][1] > x.h {
                    let cnt = x.cnt + 1;
                    ans = ans.max(cnt);
                    tree_set.insert(Envelope::new(envelopes[i][0], envelopes[i][1], cnt));
                    flag = true;
                    break;
                }
            }
            if !flag {
                tree_set.insert(Envelope::new(envelopes[i][0], envelopes[i][1], 1));
            }
            // println!("{:?}", tree_set);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use crate::arr::max_envelopes::Solution;

    #[test]
    pub fn t1() {
        let arr = vec![vec![6, 10], vec![11, 14], vec![6, 1], vec![16, 14], vec![13, 2]];
        println!("{}", Solution::max_envelopes(arr));
    }
}