use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut dp: Vec<Vec<f64>> = vec![vec![1000.0*1000.0; n+1]; 1<<n];
    dp[0][0] = 0.0;

    for i in 0..(1<<n) {
        for j in 0..n {
            for k in 0..n {
                if i & (1<<k) == 1<<k { continue; }

                let a = dp[i|(1<<k)][k];
                let b = dp[i][j] + dist(xy[j], xy[k]);

                if a > b {
                    dp[i|(1<<k)][k] = b;
                }
            }
        }
    }

    println!("{:.12}", dp[(1<<n)-1][0]);
}

fn dist(xy1: (isize, isize), xy2: (isize, isize)) -> f64 {
    let (x1, y1) = xy1;
    let (x2, y2) = xy2;

    let dx = x1 - x2;
    let dy = y1 - y2;

    return (((dx * dx) + (dy * dy)) as f64).sqrt();
}
