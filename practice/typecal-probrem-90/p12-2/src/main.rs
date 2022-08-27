use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        q:usize,
    }

    let mut uf = vec![vec![(0, 0); w + 2]; h + 2];

    for q_i in 1..=q {
        input! {
            num:usize,
        }

        if num == 1 {
            input! {
                r:usize,
                c:usize,
            }

            let root = [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .iter()
                .map(|&(y, x)| uf[y][x])
                .sorted()
                .dedup()
                .filter(|&(y, x)| y != 0 && x != 0)
                .collect_vec();

            if root.len() == 0 {
                uf[r][c] = (r, c);
            } else if root.len() == 1 {
                uf[r][c] = root[0];
            } else {
                uf[r][c] = root[0];
                for y_i in 1..=h {
                    for x_i in 1..=w {
                        if root.contains(&uf[y_i][x_i]) {
                            uf[y_i][x_i] = root[0];
                        }
                    }
                }
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
