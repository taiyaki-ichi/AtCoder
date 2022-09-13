use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        m:usize,
        mut s:[String;n],
        t:[String;m],
    }

    let l: usize = s.iter().map(|str| str.len()).sum();
    let a: Vec<Vec<String>> = (1..16 - l + 1)
        .map(|i| "_".repeat(i).to_owned())
        .permutations(n - 1)
        .collect();

    let tt: HashSet<String> = t.iter().cloned().collect();

    for ss in permutohedron::Heap::new(&mut s) {
        for aa in &a {
            let sss: String = ss
                .clone()
                .into_iter()
                .interleave(aa.clone().into_iter())
                .collect();

            //println!("{}", sss);
            if 3 <= sss.len() && sss.len() <= 16 && !tt.contains(&sss) {
                print!("{}", sss);
                return;
            }
        }
    }

    //println!("{:?}", a);
    println!("-1");
}
