use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut s:[String;n],
        t:[String;m],
    }

    let l: usize = s.iter().map(|str| str.len()).sum();
    let a: Vec<Vec<usize>> = (1..=16 - l).permutations(n - 1).collect();

    let tt: HashSet<String> = t.iter().cloned().collect();

    for ss in permutohedron::Heap::new(&mut s) {
        for aa in &a {
            let str_len: usize =
                ss.iter().map(|x| x.len()).sum::<usize>() + aa.iter().sum::<usize>();
            if str_len < 3 || 16 < str_len {
                continue;
            }

            let x: String = ss
                .clone()
                .into_iter()
                .interleave(aa.clone().into_iter().map(|i| "_".repeat(i)))
                .collect();

            if !tt.contains(&x) {
                print!("{}", x);
                return;
            }
        }
    }

    println!("-1");
}
