use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        q: usize,
        x: [usize; q],
    }

    let max = 300000; 

    let mut is_prime = vec![true; max+1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=max {
        if !is_prime[i] { continue; }

        println!("{}", i);

        for j in ((i*2)..=max).step_by(i) {
            is_prime[j] = false;   
        }
    }
}
