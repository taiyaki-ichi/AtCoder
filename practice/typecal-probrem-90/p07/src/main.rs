use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
        q:usize,
        b:[usize;q],
    }

    a.sort();

    b.iter()
        .map(|&x| {
            let i = match a.binary_search(&x) {
                Ok(x) => x,
                Err(x) => x,
            };

            let l = if i == 0 { 0 } else { i - 1 };
            let r = if i == a.len() { a.len() - 1 } else { i };

            let ll = if x > a[l] { x - a[l] } else { a[l] - x };
            let rr = if x > a[r] { x - a[r] } else { a[r] - x };

            std::cmp::min(ll, rr)
        })
        .for_each(|x| println!("{}", x));
}
