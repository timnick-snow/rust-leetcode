mod add_two_num;
mod merge_k_lists;
mod merge_k_lists2;
mod swap_pair;
mod rotate_right;
mod delete_duplicates;
mod delete_duplicates2;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn build_list_node(data: Vec<i32>) -> Option<Box<ListNode>> {
    if data.is_empty() {
        None
    } else {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut p = dummy.as_mut();
        for x in data.into_iter() {
            let node = Some(Box::new(ListNode::new(x)));
            p.as_mut().unwrap().next = node;
            p = p.unwrap().next.as_mut();
        }
        dummy.unwrap().next
    }
}

pub fn node_to_vec(mut head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut ans = Vec::new();
    while let Some(node) = head {
        ans.push(node.val);
        head = &node.next;
    }
    ans
}


#[cfg(test)]
mod test {
    use crate::link::{build_list_node, node_to_vec};

    #[test]
    pub fn t1() {
        let vec1 = vec![1, 2, 3, 4, 5];
        let head = build_list_node(vec1.clone());
        let vec2 = node_to_vec(&head);
        assert_eq!(&vec1, &vec2);
    }
}