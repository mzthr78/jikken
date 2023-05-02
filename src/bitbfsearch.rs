use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = "No";
    for b in 0..1<<n {
        let mut sum = 0;
        for i in 0..n {
            if b & (1<<i) > 0 {
                sum += a[i];
            }
        }
        if sum == k { ans = "Yes"; }
    }
    println!("{}", ans);
}
