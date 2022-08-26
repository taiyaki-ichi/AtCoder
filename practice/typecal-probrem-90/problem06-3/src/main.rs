use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        k:usize,
        s:Chars,
    }

    let (str, _) = (0..k).fold((String::new(), 0), |(str, p), i| {
        let &(curr, curr_c) = &s[p..=n - (k - i)]
            .iter()
            .enumerate()
            .min_by_key(|&(_, x)| x)
            .unwrap();
        (str + &curr_c.to_string(), p + curr + 1)
    });

    println!("{}", str);
}
