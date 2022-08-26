use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        y:usize,
        z:usize,
        a:[usize;n],
        b:[usize;n],
    }

    let math_passer = a
        .iter()
        .enumerate()
        .sorted_by_key(|&(i, v)| (Reverse(v), i))
        .take(x)
        .map(|(i, _)| i)
        .collect_vec();

    //println!("{:?}", math_passer);

    let eng_passer = b
        .iter()
        .enumerate()
        .sorted_by_key(|&(i, v)| (Reverse(v), i))
        .filter(|(i, _)| !math_passer.contains(i))
        .take(y)
        .map(|(i, _)| i)
        .collect_vec();

    //println!("{:?}", eng_passer);

    let sum_passer = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| x + y)
        .enumerate()
        .sorted_by_key(|&(i, v)| (Reverse(v), i))
        .filter(|(i, _)| !math_passer.contains(i) && !eng_passer.contains(i))
        .take(z)
        .map(|(i, _)| i)
        .collect_vec();

    let ans = math_passer
        .into_iter()
        .chain(eng_passer.into_iter())
        .chain(sum_passer.into_iter())
        .sorted();

    for i in ans {
        println!("{}", i+1);
    }
}
