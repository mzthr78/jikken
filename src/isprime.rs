use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        q: usize,
        x: [usize; q],
    }

    for xi in x {
        if is_prime(xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn is_prime(x: usize) -> bool {
    let mut i = 2;
    while (i * i) <= x {
        if x % i == 0 { return false; }
        i += 1;
     }
    return true;
}
