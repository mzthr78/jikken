use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let mut g: Vec<Vec<(usize, usize)>> = vec![vec![]; n+1];

    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }

    let mut d = vec![std::usize::MAX; n+1];
    let mut v = vec![false; n+1];

    d[1] = 0;

    loop {
        let mut x = 0;
        let mut minc = std::usize::MAX;

        for i in 1..=n {
            if v[i] { continue; }
            if d[i] < minc { 
                minc = d[i];
                x = i;
            }
        }

        if x == 0 { break; }

        v[x] = true;

        for i in 0..g[x].len() {
            let (p, c) = g[x][i];
            if v[p] { continue; }

            d[p] = min(d[p], d[x]+c);
        }
    }

    for i in 1..=n {
        println!("{}", d[i]);
    }
}
