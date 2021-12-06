use super::dp::Problem;


struct Solution(usize);


impl Problem<(Vec<Vec<i32>>, i32), bool> for Solution {
    fn solution(i: (Vec<Vec<i32>>, i32)) -> bool {
        let matrix = i.0;
        let target = i.1;
        let mut up = 0;
        let mut down = matrix.len() as i32 - 1;
        while up <= down {
            let mid = (up + down) / 2;
            let stub = matrix[mid as usize][0];
            if target == stub {
                return true;
            } else if target < stub {
                down = mid-1;
            } else {
                up = mid+1;
            }
        }
        if down < 0 {
            return false;
        }
        let row = down as usize;
        let mut left =0;
        let mut right = (matrix[row].len()-1) as i32;
        while left <= right {
            let mid = (left + right) / 2;
            let stub = matrix[row][mid as usize];
            if target == stub {
                return true;
            } else if target < stub {
                right = mid -1;
            } else {
                left = mid + 1;
            };
        }
        return false;
    }

    fn id() -> usize {
        74
    }

    fn name() -> &'static str {
        "搜索二维矩阵"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/search-a-2d-matrix/"
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{search_a_2d_matrix74::Solution, dp::Problem};

    #[test]
    fn test() {
        assert_eq!(Solution::solution((vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3)), true);
        assert_eq!(Solution::solution((vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 0)), false);
        assert_eq!(Solution::solution((vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 15)), false);
        assert_eq!(Solution::solution((vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 80)), false);
        assert_eq!(Solution::solution((vec![vec![1],vec![10],vec![23]], 20)), false);
        assert_eq!(Solution::solution((vec![], 1)), false);
        assert_eq!(Solution::solution((vec![vec![1]], 1)), true);
    }
}