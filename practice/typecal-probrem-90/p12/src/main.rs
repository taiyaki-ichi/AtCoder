use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        q:usize,
    }

    let mut masu = vec![vec![0; w + 2]; h + 2];
    let mut group = HashMap::<usize, Vec<(usize, usize)>>::new();

    for q_i in 1..=q {
        input! {
            num:usize,
        }

        if num == 1 {
            input! {
                r:usize,
                c:usize,
            }

            if masu[r][c] == 0 {
                masu[r][c] = q_i;
                group.insert(q_i, vec![(r, c)]);
            }

            let g = [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .iter()
                .map(|&(y, x)| masu[y][x])
                .sorted()
                .dedup()
                .filter(|&v| v != 0_usize)
                .collect_vec();

            for g_i in g {
                if let Some(v) = group.remove(&g_i) {
                    for (y, x) in v {
                        masu[y][x] = q_i;
                        group.get_mut(&q_i).unwrap().push((y, x));
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

            if masu[ra][ca] != 0 && masu[rb][cb] != 0 && masu[ra][ca] == masu[rb][cb] {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
