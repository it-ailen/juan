use super::{dp::Problem, merge_k_sorted_lists::ListNode};


struct Solution(usize);

impl Solution {
    fn case(nums: Vec<i32>) -> Vec<i32> {
        let input = ListNode::new_from_vec(nums);
        let output = Self::solution(input);
        ListNode::link_2_vec(&output)
    }
}

impl Problem<Option<Box<ListNode>>, Option<Box<ListNode>>> for Solution {
    fn solution(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fake = Some(Box::new(ListNode::new(0)));
        fake.as_mut().unwrap().next = head;
        let mut cur = &mut fake;
        while let Some(ref mut _cur) = cur {
            match _cur.next.take() {
                Some(mut first) => {
                    match first.next.take() {
                        Some(mut second) => {
                            first.next = second.next.take();
                            second.next = Some(first); // first
                            _cur.next = Some(second);
                            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
                        },
                        None => {
                            _cur.next = Some(first);
                            break;
                        },
                    }
                },
                None => {
                    break;
                },
            }

        }
        fake.unwrap().next
    }

    fn id() -> usize {
        24
    }

    fn name() -> &'static str {
        "两两交换链表中的节点"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/swap-nodes-in-pairs/"
    }
}



#[cfg(test)]
mod tests {
    use crate::leetcode::swap_nodes_in_pair::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::case(vec![1, 2, 3, 4, 5]), vec![2, 1, 4, 3, 5]);
        let empty: Vec<i32> = vec![];
        assert_eq!(Solution::case(vec![]), empty);
        assert_eq!(Solution::case(vec![1]), vec![1]);
    }

    #[test]
    fn literal() {

    }
}