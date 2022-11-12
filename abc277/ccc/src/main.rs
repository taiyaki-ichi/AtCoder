use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n]
    }

    let ab = ab
        .iter()
        .map(|(a, b)| if b < a { (b, a) } else { (a, b) })
        .sorted();

    let mut biru = vec![1];

    for (a, b) in ab {
        let biru_a = biru.binary_search(&a);
        let biru_b = biru.binary_search(&b);

        if biru_a.is_ok() && biru_b.is_err() {
            biru.insert(biru_b.err().unwrap(), *b);
        }

        println!("{:?}",biru);
    }

    println!("{}", biru.last().unwrap());
}
