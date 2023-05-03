use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        w: usize,
        uv: [(usize, isize); n],
    }

    let mut dp = vec![vec![-1; w+1]; n+1];

    dp[0][0] = 0;

    for i in 1..=n {
        let (weight, value) = uv[i-1];
        for j in 0..=w {
            if j < weight || dp[i-1][j-weight] == -1 {
                dp[i][j] = dp[i-1][j]; 
            } else {
                dp[i][j] = max(dp[i-1][j], dp[i-1][j-weight] + value);
            }
        }
    }

    for i in 0..=n {
        println!("{:?}", dp[i]);
    }
}
