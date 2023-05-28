use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    let mut b = vec![0; m];

    for i in 0..m {
        for j in 0..n {
            b[i] |= a[i][j]<<j;
        }
    }

    let mut dp = vec![1<<10; 1<<n];

    for i in 0..m {
        dp[b[i]] = 1;

        for j in 0..1<<n {
            dp[j|b[i]] = min(dp[j|b[i]], dp[j]+1);
        }

        //println!("{:?}", dp);
    }

    if dp[(1<<n)-1] == 1<<10 { dp[(1<<n)-1] = -1; }

    println!("{}", dp[(1<<n)-1]);
}
