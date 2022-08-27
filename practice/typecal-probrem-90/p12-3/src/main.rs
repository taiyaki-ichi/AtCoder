use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        q:usize,
    }

    let mut uf = vec![vec![(0, 0); w + 2]; h + 2];

    for _ in 1..=q {
        input! {
            num:usize,
        }

        if num == 1 {
            input! {
                r:usize,
                c:usize,
            }

            let roots = [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .iter()
                .map(|&(y, x)| uf[y][x])
                .sorted()
                .dedup()
                .filter(|&(y, x)| y != 0 && x != 0)
                .collect_vec();

            if roots.len() == 0 {
                uf[r][c] = (r, c);
            } else if roots.len() == 1 {
                uf[r][c] = roots[0];
            } else {
                uf[r][c] = roots[0];
                uf.iter_mut().flatten().for_each(|e| {
                    if roots.contains(e) {
                        *e = roots[0];
                    }
                });
            }
        } else if num == 2 {
            input! {
                ra:usize,
                ca:usize,
                rb:usize,
                cb:usize,
            }

            if uf[ra][ca] != (0, 0) && uf[rb][cb] != (0, 0) && uf[ra][ca] == uf[rb][cb] {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
