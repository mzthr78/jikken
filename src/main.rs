use proconio::{fastout, input};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        abcd: [(usize, usize, isize, isize); m],
    }

    let mut g: Vec<Vec<(usize, isize, isize)>> = vec![vec![]; n+1];

    for (a, b, c, d) in abcd {
        g[a].push((b, c * 10000 - d, d));
        g[b].push((a, c * 10000 - d, d));
    }

    let mut bh = BinaryHeap::new();
    let mut v = vec![-1; n+1];
    let mut t: Vec<isize> = vec![0; n+1];

    bh.push(Reverse((0, 1, 0)));

    while !bh.is_empty() {
        let (cost, pos, tree) = bh.pop().unwrap().0;
        if v[pos] < 0 {
            v[pos] = cost;
            t[pos] = tree;
        } else {
            continue;
        }

        for i in 0..g[pos].len() {
            let (p, c, s) = g[pos][i];

            if v[p] >= 0 { continue; }

            bh.push(Reverse((v[pos] + c, p, s + t[pos])));
        }
    }

    println!("{} {}", (v[n] + t[n]) / 10000, t[n]);
}
