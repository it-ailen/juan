use super::dp::Problem;


struct Solution(usize);

impl Solution {
    fn case(src: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut src = src.clone();
        Self::solution(&mut src);
        src
    }
}

impl Problem<&mut Vec<Vec<i32>>, ()> for Solution {
    fn solution(matrix: &mut Vec<Vec<i32>>) -> () {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return;
        }
        let mut col_2_clear = vec![false; matrix[0].len()];
        for r in 0..matrix.len() {
            let row = &mut matrix[r];
            let mut row_2_clear = false;
            for (col, &n) in row.iter().enumerate() {
                if n == 0 {
                    row_2_clear = true;
                    col_2_clear[col] = true;
                }
            }
            if row_2_clear {
                for n in row {
                    *n = 0;
                }
            }
        }
        for (col, &clear) in col_2_clear.iter().enumerate() {
            if clear {
                for row in 0..matrix.len() {
                    matrix[row][col] = 0;
                }
            }
        }
    }

    fn id() -> usize {
        73
    }

    fn name() -> &'static str {
        "矩阵置零"
    }

    fn url() -> &'static str {
        "https://leetcode-cn.com/problems/set-matrix-zeroes/"
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::{set_matrix_zeroes73::Solution};

    #[test]
    fn test() {
        assert_eq!(Solution::case(vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]]), vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]]);
        assert_eq!(Solution::case(vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]]), vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]]);
    }
}