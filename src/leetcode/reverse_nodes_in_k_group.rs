use super::{dp::Problem, merge_k_sorted_lists::ListNode};



struct Solution(usize);

impl Solution {
    fn case(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let list = ListNode::new_from_vec(nums);
        let result = Self::solution((list, k));
        ListNode::link_2_vec(&result)
    }

    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.take();
        let mut prev = None;
        while let Some(mut _cur) = current.take() {
            let next = _cur.next.take();
            _cur.next = prev.take();
            prev = Some(_cur);
            current = next;
        }
        prev.take()
    }

    /// reverse k-nodes and return the tail nodes.
    fn reverse_k(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = &mut head;
        while let Some(_cur) = cur {
            cur = &mut _cur.next;
            len += 1;
            if len >= k {
                break;
            }
        }
        if len < k {
            return head;
        }
        let mut tail = Self::reverse_k(cur.take(), k);
        let mut head = Self::reverse(head);
        let mut cur = &mut head;
        loop {
            if let Some(ref mut _cur) = cur {
                cur = &mut _cur.next;
            } else {
                break;
            }
        };
        *cur = tail.take();
        head
    }
}

impl Problem<(Option<Box<ListNode>>, i32), Option<Box<ListNode>>> for Solution {
    fn solution(i: (Option<Box<ListNode>>, i32)) -> Option<Box<ListNode>> {
        Self::reverse_k(i.0, i.1)
    }

    fn id() -> usize {
        25
    }

    fn name() -> &'static str {
        "K 个一组翻转链表"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/reverse-nodes-in-k-group/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::reverse_nodes_in_k_group::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::case(vec![1, 2, 3, 4, 5], 2), vec![2, 1, 4, 3, 5]);
        assert_eq!(Solution::case(vec![1, 2, 3, 4, 5], 3), vec![3, 2, 1, 4, 5]);
        assert_eq!(Solution::case(vec![1, 2, 3, 4, 5], 1), vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::case(vec![1, 2, 3, 4, 5], 4), vec![4, 3, 2, 1, 5]);
        assert_eq!(Solution::case(vec![1, 2, 3, 4, 5], 5), vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::case(vec![1, 2, 3, 4, 5], 6), vec![1, 2, 3, 4, 5]);
    }
}