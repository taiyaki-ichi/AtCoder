use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        g:[Chars;h],
    }

    let mut arrived = vec![vec![false; w]; h];

    let mut pos = (0, 0);

    loop {
        // println!("{} {} {}",pos.0+1,pos.1+1,g[pos.0][pos.1]);

        match g[pos.0][pos.1] {
            'U' => {
                if pos.0 == 0 {
                    break;
                } else {
                    pos = (pos.0 - 1, pos.1);
                }
            }

            'D' => {
                if pos.0 == h - 1 {
                    break;
                } else {
                    pos = (pos.0 + 1, pos.1);
                }
            }

            'L' => {
                if pos.1 == 0 {
                    break;
                } else {
                    pos = (pos.0, pos.1 - 1);
                }
            }

            'R' => {
                if pos.1 == w - 1 {
                    break;
                } else {
                    pos = (pos.0, pos.1 + 1);
                }
            }

            _ => {
                return;
            }
        }

        if arrived[pos.0][pos.1] {
            println!("-1");
            return;
        } else {
            arrived[pos.0][pos.1] = true;
        }
    }

    println!("{} {}",pos.0+1,pos.1+1);
}
