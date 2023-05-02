use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let mut mf = MaxFlow::new(n);

    for (a, b, c) in abc {
        mf.g[a].push(b);
        mf.g[b].push(a);
        mf.e[a][b] = c;
    }

    println!("{}", mf.max_flow(n));
}

struct MaxFlow {
    g: Vec<Vec<usize>>,
    e: Vec<Vec<usize>>,
    v: Vec<bool>,
}

impl MaxFlow {
    fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; n+1],
            e: vec![vec![0; n+1]; n+1],
            v: vec![false; n+1],
        }
    }

    fn dfs(&mut self, from: usize, goal: usize, maxflow: usize) -> usize {
        if from == goal { return maxflow; }

        self.v[from] = true;

        for i in 0..self.g[from].len() {
            let to = self.g[from][i];
            if self.v[to] { continue; }

            let cap = self.e[from][to];
            if cap == 0 { continue; }

            let flow = self.dfs(to, goal, min(maxflow, cap));

            if flow > 0 {
                self.e[from][to] -= flow;
                self.e[to][from] += from;
                return flow;
            }
        }
        return 0;
    }

    fn max_flow(&mut self, n: usize) -> usize {
        let mut max_flow = 0;
        loop {
            for i in 0..=n {
                self.v[i] = false;
            }
            let tmp = self.dfs(1, n, std::usize::MAX);
            max_flow += tmp;

            if tmp == 0 { break; }
        }
        return max_flow;
    }
}
