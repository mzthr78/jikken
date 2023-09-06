use proconio::{fastout, input};
use std::cmp::max;

struct SegT {
    s: usize,
    v: Vec<usize>,
}

impl SegT {
    fn new(n: usize) -> SegT {
        let mut tmp = 1;

        while tmp < n {
            tmp *= 2;
        }

        SegT {
            s: tmp,
            v: vec![0; tmp*2],
        }
    }

    fn update(&mut self, mut pos: usize, x: usize) {
        pos = pos + self.s - 1;

        self.v[pos] = x;

        while pos > 1 {
            pos /= 2;

            self.v[pos] = max(self.v[pos*2], self.v[pos*2+1]);
        }
    }

    fn query(&self, l: usize, r: usize, s: usize, e: usize, i: usize) -> isize {
        if r <= s || e <= l {
            return -1;
        } else if l <= s && e <= r {
            return self.v[i] as isize;
        } else {
            let m = (s + e) / 2;
            let ansl = self.query(l, r, s, m, i * 2);
            let ansr = self.query(l, r, m, e, i * 2 + 1);
            return max(ansl, ansr);
        }
    }
}

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
    }

    let mut s = SegT::new(n);

    for _ in 0..m {
        input!{
            q: usize,
        }

        if q == 1 {
            input!{
                pos: usize,
                x: usize,
            }
            s.update(pos, x);
        } else if q == 2 {
            input!{
                l: usize,
                r: usize,
            }
            //let ans = s.query(l, r, 1, s.s, 1);
            let ans = s.query(l, r, 1, s.s + 1, 1);
            println!("{}", ans);
        }
    }
}
