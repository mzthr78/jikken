use proconio::{fastout, input};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        abc: [(usize, usize, isize); m],
    }

    let mut g: Vec<Vec<(usize, isize)>> = vec![vec![]; n+1];

    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }

    let mut bh = BinaryHeap::new();
    let mut v = vec![-1; n+1];

    bh.push(Reverse((0, 1)));

    while !bh.is_empty() {
        let (cost, pos) = bh.pop().unwrap().0;
        if v[pos] >= 0 { continue; }
        v[pos] = cost;

        for i in 0..g[pos].len() {
            let (p, c) = g[pos][i];
            if v[p] >= 0 { continue; }
            bh.push(Reverse((cost+c, p)));
        }
    }

    for i in 1..v.len() {
        println!("{}", v[i]);
    }
}
