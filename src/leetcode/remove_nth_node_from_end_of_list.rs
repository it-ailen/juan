use super::{dp::Problem, merge_k_sorted_lists::ListNode};


struct Solution(usize);

impl Solution {
    fn case(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let list = ListNode::new_from_vec(nums);
        let result = Self::solution((list, n));
        ListNode::link_2_vec(&result)
    }
}

impl Problem<(Option<Box<ListNode>>, i32), Option<Box<ListNode>>> for Solution {
    fn solution(i: (Option<Box<ListNode>>, i32)) -> Option<Box<ListNode>> {
        let mut head = i.0;
        let n = i.1 as usize;
        let mut len = 0;
        let mut cursor = &head;
        while let Some(_cur) = cursor {
            len += 1;
            cursor = &_cur.next;
        }
        let mut cursor = &mut head;
        for _ in 0..(len-n) {
            cursor = &mut cursor.as_mut()?.next;
        }
        *cursor = cursor.take()?.next;
        head
    }

    fn id() -> usize {
        19
    }

    fn name() -> &'static str {
       "删除链表的倒数第 N 个结点" 
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::remove_nth_node_from_end_of_list::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::case(vec![1, 2, 3, 4], 2), vec![1, 2, 4]);
    }
}