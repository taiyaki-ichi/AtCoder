use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        q:usize,
    }

    let mut uf = UnionFind::<usize>::new((h + 2) * (w + 2));
    let mut masu = vec![false; (h + 2) * (w + 2)];
    let to_index = |r, c| r * (h + 2) + c;

    for _ in 1..=q {
        input! {
            num:usize,
        }

        if num == 1 {
            input! {
                r:usize,
                c:usize,
            }

            masu[to_index(r, c)] = true;

            [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                .iter()
                .filter(|&&(y, x)| masu[to_index(y, x)])
                .for_each(|&(y, x)| {
                    uf.union(to_index(r, c), to_index(y, x));
                });
        } else if num == 2 {
            input! {
                ra:usize,
                ca:usize,
                rb:usize,
                cb:usize,
            }

            let b = masu[to_index(ra, ca)]
                && masu[to_index(rb, cb)]
                && uf.equiv(to_index(ra, ca), to_index(rb, cb));
            println!("{}", if b { "Yes" } else { "No" });
        }
    }
}
