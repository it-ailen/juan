use super::dp::Problem;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn new_from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        for i in nums {
            if let Some(ref mut _tail) = tail {
                _tail.next = Some(Box::new(ListNode::new(i)));
                tail = &mut _tail.next;
            }
        }
        head.unwrap().next.take()
    }

    pub fn link_2_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut cur = list;
        while let Some(ref _cur) = cur {
            result.push(_cur.val);
            cur = &_cur.next;
        }
        result
    } 
}

struct Solution(usize);

impl Solution {
    fn link_2_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut cur = list;
        while let Some(ref _cur) = cur {
            result.push(_cur.val);
            cur = &_cur.next;
        }
        result
    }
    fn parse_link(l: Vec<i32>) -> Box<ListNode> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut head;
        for i in l {
            if let Some(ref mut _cur) = cur {
                _cur.next = Some(Box::new(ListNode::new(i)));
                cur = &mut _cur.next;
            }
        }
        let result = head.unwrap().next.take();
        result.unwrap()
    }

    fn parse_list(links: Vec<Vec<i32>>) -> Vec<Option<Box<ListNode>>> {
        let mut result: Vec<Option<Box<ListNode>>> = Vec::new();
        for each in links {
            result.push(Some(Self::parse_link(each)));
        }
        result
    }

    fn case(lists: Vec<Vec<i32>>) -> Option<Box<ListNode>> {
        Self::solution(Self::parse_list(lists))
    }

    fn merge(lists: &Vec<Option<Box<ListNode>>>, l: usize, r: usize) -> Option<Box<ListNode>> {
        if l == r {
            lists[l].clone()
        } else {
            let mid = (l + r) / 2;
            let left = Self::merge(lists, l, mid);
            let right = Self::merge(lists, mid + 1, r);
            let mut head = Some(Box::new(ListNode::new(0)));
            let mut tail = &mut head;
            let mut left_cursor = left;
            let mut right_cursor = right;
            let mut next = true;
            while next {
                // 取出值来遍历
                match (left_cursor.take(), right_cursor.take()) {
                    (None, None) => {
                        next = false;
                    }
                    (None, Some(r)) => {
                        // l 已经完了，把 r 后的链表接到结果链表中就行
                        // 使用 ref 禁止移动；mut 确保可以修改
                        if let Some(ref mut _cur) = tail {
                            _cur.next = Some(r);
                        }
                        next = false;
                    }
                    (Some(l), None) => {
                        if let Some(ref mut _cur) = tail {
                            _cur.next = Some(l);
                        }
                        next = false;
                    }
                    (Some(mut l), Some(mut r)) => {
                        if &l.val <= &r.val {
                            // take 将 Option 换出，原来值换成 None
                            let _next = l.next.take();
                            if let Some(ref mut _cur) = tail {
                                _cur.next = Some(l);
                                tail = &mut _cur.next;
                            }
                            left_cursor = _next;
                            right_cursor = Some(r); // 放回
                        } else {
                            let _next = r.next.take();
                            if let Some(ref mut _cur) = tail {
                                _cur.next = Some(r);
                                tail = &mut _cur.next;
                            }
                            right_cursor = _next;
                            left_cursor = Some(l); // 放回
                        }
                    }
                }
            }
            head.unwrap().next.take()
        }
    }
}

impl Problem<Vec<Option<Box<ListNode>>>, Option<Box<ListNode>>> for Solution {
    fn solution(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let len = lists.len();
        if len == 0 {
            None
        } else {
            Self::merge(&lists, 0, len - 1)
        }
    }

    fn id() -> usize {
        23
    }

    fn name() -> &'static str {
        "合并K个升序链表"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/merge-k-sorted-lists/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::merge_k_sorted_lists::Solution;

    #[test]
    fn test() {
        assert!(Solution::case(vec![]).is_none());
        assert_eq!(
            Solution::link_2_vec(&Solution::case(vec![
                vec![1, 4, 5],
                vec![1, 3, 4],
                vec![2, 6]
            ])),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
        // assert_eq!(Solution::case(vec![vec![1,4,5], vec![1,3,4], vec![2, 6]]).unwrap(), vec![])
    }
}
