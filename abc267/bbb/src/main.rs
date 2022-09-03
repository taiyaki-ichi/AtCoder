use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }

    let ss = [
        s[6],
        s[3],
        if s[7] == '0' && s[1] == '0' { '0' } else { '1' },
        if s[4] == '0' && s[0] == '0' { '0' } else { '1' },
        if s[8] == '0' && s[2] == '0' { '0' } else { '1' },
        s[5],
        s[9],
    ];

    let sss = ss.iter().dedup().collect_vec();
    for i in 1..sss.len() - 1 {
        if s[0] == '0' && *sss[i - 1] == '1' && *sss[i] == '0' && *sss[i + 1] == '1' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
