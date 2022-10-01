use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        q:usize,
    }

    let mut nums = vec![vec![0]; n];

    for i in 0..n {
        input! {
            l:usize,
        }

        input! {
            a:[usize;l],
        }

        nums[i] = a.into_iter().collect_vec();
    }

    input! {
        st:[(usize,usize);q],
    }

    st.iter()
        .for_each(|&(s, t)| println!("{}", nums[s - 1][t - 1]));
}
