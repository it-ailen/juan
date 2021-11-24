use crate::leetcode::dp::Problem;

pub struct Input(String, i32);

/// https://leetcode-cn.com/problems/zigzag-conversion/solution/z-zi-xing-bian-huan-by-leetcode/
pub struct Zigzag(i32);

impl Zigzag {
    fn row(idx: usize, r: usize) -> usize {
        let stub = 2 * (r - 1); // 遇到这个就回绕；并在一个周期内以r-1对称
        let first_cycle = idx % stub;
        if first_cycle < r {
            first_cycle % r
        } else {
            2 * (r - 1) - first_cycle
        }
    }

    fn col(idx: usize, r: usize) -> usize {
        let stub = 2 * (r - 1); // 遇到这个就回绕；并在一个周期内以r-1对称
        let t = idx % stub;
        let v: usize = if t < r { 0 } else { t - (r - 1) };
        // println!("col:: idx={:?} row={:?} t={:?} v={:?}", idx, r, t, v);
        (idx) / stub * (r - 1) + v
    }
}

impl Problem<Input, String> for Zigzag {
    fn solution(i: Input) -> String {
        let s = i.0.as_bytes();
        let row = i.1;
        let len = s.len();
        let max_clo = Self::col(len - 1, row as usize) + 1;
        let mut vector = vec![vec![' ' as u8; max_clo]; row as usize];
        for (idx, &ch) in s.iter().enumerate() {
            let r = Self::row(idx, row as usize);
            let c = Self::col(idx, row as usize);
            // println!("row={:?} idx={:?} r={:?} c={:?}", row, idx, r, c);
            vector[r][c] = ch;
        }
        let mut builder = "".to_owned();
        for r in 0..row as usize {
            builder.push_str(&std::str::from_utf8(&vector[r]).unwrap());
            builder.push('\n');
            // println!("{:?}", std::str::from_utf8(&vector[r]));
            // for c in 0..max_clo {
            //     print!()
            // }
        }
        builder
    }

    fn id() -> usize {
        6
    }

    fn name() -> &'static str {
        "Z 字形变换"
    }

    fn url() -> &'static str {
        todo!()
    }

    
}

#[cfg(test)]
mod tests {
    use crate::leetcode::{
        dp::Problem,
        transform::zigzag::{Input, Zigzag},
    };

    #[test]
    fn base() {
        println!(
            "{:?}",
            Zigzag::solution(Input("PAYPALISHIRING".to_string(), 3))
        );

        println!(
            "{:?}",
            Zigzag::solution(Input("PAYPALISHIRING".to_string(), 4))
        );
        // assert_eq!(
        //     Zigzag::solution(Input(("PAYPALISHIRING".to_string(), 3))),
        //     "".to_string()
        // );
    }

    #[test]
    fn test_row() {
        assert_eq!(Zigzag::row(0, 3), 0);
        assert_eq!(Zigzag::row(1, 3), 1);
        assert_eq!(Zigzag::row(2, 3), 2);
        assert_eq!(Zigzag::row(3, 3), 1);
        assert_eq!(Zigzag::row(4, 3), 0);
        assert_eq!(Zigzag::row(4, 4), 2);
        assert_eq!(Zigzag::row(5, 4), 1);
        assert_eq!(Zigzag::row(6, 4), 0);
    }

    #[test]
    fn test_col() {
        assert_eq!(Zigzag::col(0, 3), 0);
        assert_eq!(Zigzag::col(2, 3), 0);
        assert_eq!(Zigzag::col(3, 3), 1);
        assert_eq!(Zigzag::col(4, 3), 2);
        assert_eq!(Zigzag::col(6, 3), 2);
        assert_eq!(Zigzag::col(6, 4), 3);
        assert_eq!(Zigzag::col(7, 4), 3);
    }
}
