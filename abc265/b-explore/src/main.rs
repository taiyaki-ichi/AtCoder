use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut t:usize,
        a:[usize;n-1],
        xy:[(usize,usize);m],
    }

    let mut bs = HashMap::new();
    for (x, y) in xy {
        bs.insert(x, y);
    }

    for i in 0..n - 1 {
        // println!("{}", t);

        if let Some(b) = bs.get(&(i + 1)) {
            t += *b;
        }

        if t <= a[i] {
            println!("No");
            return;
        }

        t -= a[i];
    }

    println!("Yes");
}
