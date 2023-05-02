use proconio::{fastout, input};

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

    let mut r = n;
    let mut v = vec![-1; n+1];
    let mut d: Vec<usize> = vec![];

    v[1] = 0;
    d.push(1);
    r -= 1;

    loop {
        let mut pos = 0;
        let mut cost = std::isize::MAX;
        for i in 0..d.len() {
            for &(p, c) in &g[d[i]] {
                if v[p] >= 0 { continue; }
                if v[d[i]] + c < cost {
                    pos = p;
                    cost = v[d[i]]+c;
                }
            }
        }
        d.push(pos);
        v[pos] = cost;

        r -= 1;
        if r <= 0 { break; }
    }

    for i in 1..=n {
        println!("{}", v[i]);
    }
}
