use itertools::Itertools;
use permutohedron::LexicalPermutation;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut s:[String;n],
        mut t:[String;m],
    }

    let l: usize = s.iter().map(|str| str.len()).sum();
    let a: Vec<Vec<usize>> = (1..=16 + 1 + 1 - l - n)
        .combinations_with_replacement(n - 1)
        .filter(|x| {
            let ll = x.iter().sum::<usize>() + l;
            3 <= ll && ll <= 16
        })
        .collect();

    t.sort();
    s.sort();

    while {
        for aa in &a {
            let x: String = s
                .clone()
                .into_iter()
                .interleave(aa.clone().into_iter().map(|i| "_".repeat(i)))
                .collect();

            if t.binary_search(&x).is_err() {
                print!("{}", x);
                return;
            }
        }

        s.next_permutation()
    }{}

    println!("-1");
}
