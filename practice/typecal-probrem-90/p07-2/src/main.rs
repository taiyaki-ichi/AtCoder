use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
        q:usize,
        b:[i64;q],
    }

    a.sort();

    b.iter()
        .map(|&x| match a.binary_search(&x) {
            Ok(a_i) => (a[a_i] - x).abs(),
            Err(a_i) => {
                let a_l = if a_i == 0 { 0 } else { a_i - 1 };
                let a_r = if a_i == a.len() { a.len() - 1 } else { a_i };
                std::cmp::min((a[a_l] - x).abs(), (a[a_r] - x).abs())
            }
        })
        .for_each(|x| println!("{}", x));
}
