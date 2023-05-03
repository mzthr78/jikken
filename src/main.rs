use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut td: [(usize, usize); n],
    }

    let mut dp = vec![vec![-1; 1440+1]; n+1];

    td.sort_by_key(|td| td.1);
    dp[0][0] = 0;
    
    for i in 1..=n {
        let (t, d) = td[i-1];
        for j in 0..=1440 {
            //if j >= t && j <= d && dp[i-1][j-t] >= 0 {
            if j >= t && j <= d {
                dp[i][j] = max(dp[i-1][j], dp[i-1][j-t]+1);
            } else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }

    dp[n].sort();
    println!("{}", dp[n][1440]);
}
