
use proconio::input;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n-1]
    }

    let mut path = vec![vec![]; n];
    for (a, b) in ab {
        path[a - 1].push(b - 1);
        path[b - 1].push(a - 1);
    }

    let (edge, _) = calc_max_len(0, 0, &path);
    let (_, len) = calc_max_len(edge, edge, &path);

    println!("{:?}", len + 1);
}

fn calc_max_len(prev: usize, curr: usize, path: &Vec<Vec<usize>>) -> (usize, usize) {
    if let Some(result) = path[curr]
        .iter()
        .filter(|i| **i != prev)
        .map(|i| {
            let (v, len) = calc_max_len(curr, *i, path);
            (v, len + 1)
        })
        .max_by_key(|(_, len)| *len)
    {
        result
    } else {
        (curr, 0)
    }
}
