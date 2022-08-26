use proconio::input;

fn main() {
    input! {
        n:usize,
        cp:[(usize,usize);n],
        q:usize,
        lr:[(usize,usize);q],
    }

    let sum = cp
        .iter()
        .fold(vec![(0,0)], |mut v, &(c, p)| {
            let (prev_x, prev_y) = v[v.len()-1];
            if c == 1 {
                v.push((prev_x + p, prev_y))
            } else {
                v.push((prev_x, p + prev_y))
            };
            v
        });

    lr.iter()
        .map(|&(l, r)| (sum[r].0 - sum[l - 1].0, sum[r].1 - sum[l - 1].1))
        .for_each(|(x, y)| println!("{} {}", x, y));
}
