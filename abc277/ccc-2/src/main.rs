use proconio::input;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n]
    }

    let m = ab
        .iter()
        .fold(0, |acc, &(a, b)| std::cmp::max(acc, std::cmp::max(a, b)));

    let mut uf = vec![(1, 1)];

    for (a, b) in ab {
        let uf_a = uf.binary_search_by(|v| v.0.cmp(&a));
        let uf_b = uf.binary_search_by(|v| v.0.cmp(&b));

        let i = std::cmp::max(a, b);

        if uf_a.is_ok() {
            uf[uf_a.unwrap()].1 = i;
        }
        if uf_b.is_ok() {
            uf[uf_b.unwrap()].1 = i;
        }

        if uf_a.is_err() && uf_b.is_err() {
            if a < b {
                uf.insert(uf_b.unwrap(), (uf_b.unwrap(), i));
                uf.insert(uf_a.unwrap(), (uf_a.unwrap(), i));
            } else {
                uf.insert(uf_a.unwrap(), (uf_a.unwrap(), i));
                uf.insert(uf_b.unwrap(), (uf_b.unwrap(), i));
            }
        } else if uf_a.is_err() {
            uf.insert(uf_a.unwrap(), (uf_a.unwrap(), i));
        } else if uf_b.is_err() {
            uf.insert(uf_b.unwrap(), (uf_b.unwrap(), i));
        }
    }

    let ans=uf.binary_search((1,_));
    println!("{}", &ans);
}
