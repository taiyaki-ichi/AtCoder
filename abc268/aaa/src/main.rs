use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        nums:[usize;5],
    }

    let ans = nums.iter().sorted().dedup().count();

    println!("{}", ans);
}
