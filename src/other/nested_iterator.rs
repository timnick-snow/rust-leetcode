#![allow(dead_code)]
/*
341. 扁平化嵌套列表迭代器
中等
相关标签
相关企业
给你一个嵌套的整数列表 nestedList 。每个元素要么是一个整数，要么是一个列表；该列表的元素也可能是整数或者是其他列表。请你实现一个迭代器将其扁平化，使之能够遍历这个列表中的所有整数。

实现扁平迭代器类 NestedIterator ：

NestedIterator(List<NestedInteger> nestedList) 用嵌套列表 nestedList 初始化迭代器。
int next() 返回嵌套列表的下一个整数。
boolean hasNext() 如果仍然存在待迭代的整数，返回 true ；否则，返回 false 。
你的代码将会用下述伪代码检测：

initialize iterator with nestedList
res = []
while iterator.hasNext()
    append iterator.next() to the end of res
return res
如果 res 与预期的扁平化列表匹配，那么你的代码将会被判为正确。



示例 1：

输入：nestedList = [[1,1],2,[1,1]]
输出：[1,1,2,1,1]
解释：通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,1,2,1,1]。
示例 2：

输入：nestedList = [1,[4,[6]]]
输出：[1,4,6]
解释：通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,4,6]。


提示：

1 <= nestedList.length <= 500
嵌套列表中的整数值在范围 [-106, 106] 内
 */

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    list: Vec<NestedInteger>,
    size: usize,
    idx: usize,
    nested: Option<Box<NestedIterator>>,
}


/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(list: Vec<NestedInteger>) -> Self {
        let size = list.len();
        NestedIterator {
            list,
            size,
            idx: 0,
            nested: None,
        }
    }

    fn next(&mut self) -> i32 {
        if let Some(it) = &mut self.nested {
            let val = it.next();
            if !self.nested.as_mut().unwrap().has_next() {
                self.nested.take();
            }
            val
        } else {
            let integer = std::mem::replace(&mut self.list[self.idx], NestedInteger::Int(0));
            self.idx += 1;
            match integer {
                NestedInteger::Int(val) => val,
                NestedInteger::List(_) => unreachable!()
            }
        }
    }

    fn has_next(&mut self) -> bool {
        if self.nested.is_some() {
            return true;
        }
        if self.idx == self.size {
            return false;
        }
        let integer = std::mem::replace(&mut self.list[self.idx], NestedInteger::Int(0));
        match integer {
            NestedInteger::Int(_) => {
                let _ = std::mem::replace(&mut self.list[self.idx], integer);
                true
            }
            NestedInteger::List(nexted_list) => {
                let mut iterator = NestedIterator::new(nexted_list);
                self.idx += 1;
                let has = iterator.has_next();
                if has {
                    self.nested = Some(Box::new(iterator));
                }
                has || self.has_next()
            }
        }
    }
}
