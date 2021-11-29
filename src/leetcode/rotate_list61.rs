use super::{dp::Problem, merge_k_sorted_lists::ListNode};


struct Solution(usize);

impl Solution {
    fn case(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        None
    }
}

impl Problem<(Vec<i32>, i32), Vec<i32>> for Solution {
    fn solution(i: (Vec<i32>, i32)) -> Vec<i32> {
        todo!()
    }

    fn id() -> usize {
        61
    }

    fn name() -> &'static str {
        "旋转链表"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/rotate-list/"
    }
}