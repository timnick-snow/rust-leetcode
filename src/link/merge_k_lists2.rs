#![allow(dead_code)]
struct Solution;
use crate::link::ListNode;

use std::{
    cmp::{Ord, PartialOrd, Reverse},
    collections::BinaryHeap,
};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut ans = None;
        let mut cur = &mut ans;
        let mut heap = lists.into_iter()
            .filter_map(|x| x)
            .map(|x| Reverse(x))
            .collect::<BinaryHeap<_>>();

        while let Some(mut x) = heap.pop() {
            if let Some(y) = x.0.next.take() {
                heap.push(Reverse(y));
            }
            cur = &mut cur.insert(x.0).next;
        }
        ans
    }
}