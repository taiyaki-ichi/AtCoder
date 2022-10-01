use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }

    let mut a = a.into_iter().sorted().dedup().collect_vec();
    let mut amari = n - a.iter().count();

    let mut i = 0;
    while a.iter().count() > 0 && (a.last().unwrap() >= &i || amari > 0) {
        // println!("{:?}", a);
        // println!("{}", amari);
        match a.binary_search(&(i + 1)) {
            Ok(_) => {}
            Err(u_i) => {
                if amari >= 2 {
                    amari -= 2;
                } else if amari == 1 && a.iter().count() >= 1 && a[a.iter().count() - 1] > i + 1 {
                    amari -= 1;
                    a.pop();
                } else if a.iter().count() >= 2
                    && a[a.iter().count() - 1] > i + 1
                    && a[a.iter().count() - 2] > i + 1
                {
                    a.pop();
                    a.pop();
                } else {
                    break;
                }
            }
        }

        i += 1;
    }

    println!("{}", i);
}
