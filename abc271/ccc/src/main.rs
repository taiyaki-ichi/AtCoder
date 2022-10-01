use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }

    let mut a = a.into_iter().sorted().collect_vec();

    let mut i = 0;
    while a.iter().count() > 0 {
        println!("{:?}", a);
        if let Err(u_i) = a.binary_search(&(i + 1)) {
            if u_i <= a.iter().count() - 2 {
                a.pop();
                a.pop();
            } else {
                break;
            }
        }

        match a.binary_search(&(i + 1)) {
            Err(u_i)=>{
                if u_i <= a.iter().count() - 2 {
                    a.pop();
                    a.pop();
                } else {
                    break;
                }
            },
            Ok(mut s)=>{
                while s<a.iter().count()-1{
                    if a[s]==i+1&&a[s]
                }
            },
        }

        i += 1;
    }

    println!("{}", i);
}
