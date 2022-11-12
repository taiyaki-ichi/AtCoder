use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        p:[usize;n],
    }

    let ans = p.iter().find_position(|&&v| v == x).unwrap().0 + 1;

    println!("{}", ans);
}
