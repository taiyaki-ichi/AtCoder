use itertools::Itertools;
use proconio::input;

fn can_move_pos(
    (pos_x, pos_y): (usize, usize),
    (move_x, move_y): (usize, usize),
    n: usize,
) -> Vec<(usize, usize)> {
    let mut ans = vec![];
    if pos_x >= move_x && pos_y >= move_y {
        ans.push((pos_x - move_x, pos_y - move_y));
    }
    if pos_x >= move_x && pos_y + move_y < n {
        ans.push((pos_x - move_x, pos_y + move_y));
    }
    if pos_x + move_x < n && pos_y >= move_y {
        ans.push((pos_x + move_x, pos_y - move_y));
    }
    if pos_x + move_x < n && pos_y + move_y < n {
        ans.push((pos_x + move_x, pos_y + move_y));
    }

    return ans;
}

fn dps(
    (pos_x, pos_y): (usize, usize),
    mov: &Vec<(usize, usize)>,
    mut masu: Vec<Vec<i64>>,
    n: usize,
) -> Vec<Vec<i64>> {
    let next_v = masu[pos_y][pos_x] + 1;
    let next_pos_list = mov
        .iter()
        .map(|&v| can_move_pos((pos_x, pos_y), v, n))
        .concat();

    for next_pos in next_pos_list {
        if masu[next_pos.1][next_pos.0] < 0 || masu[next_pos.1][next_pos.0] > next_v {
            masu[next_pos.1][next_pos.0] = next_v;
            masu = dps(next_pos, mov, masu, n);
        }
    }

    return masu;
}

fn bps(
    pos: Vec<(usize, usize)>,
    mov: &Vec<(usize, usize)>,
    mut masu: Vec<Vec<i64>>,
    n: usize,
) -> Vec<Vec<i64>> {
    let mut result_pos = vec![(0, 0); 0];
    for (pos_x, pos_y) in pos {
        let next_v = masu[pos_y][pos_x] + 1;
        let next_pos_list = mov
            .iter()
            .map(|&v| can_move_pos((pos_x, pos_y), v, n))
            .concat();

        for next_pos in next_pos_list {
            if masu[next_pos.1][next_pos.0] < 0 || masu[next_pos.1][next_pos.0] > next_v {
                masu[next_pos.1][next_pos.0] = next_v;
                result_pos.push(next_pos);
            }
        }
    }

    if result_pos.iter().count() == 0 {
        return masu;
    } else {
        return bps(result_pos, mov, masu, n);
    }
}

fn main() {
    input! {
        n:usize,
        m:usize,
    }

    let mut mov = vec![(0, 0); 0];
    for i in 0..n {
        for j in 0..n {
            if i * i + j * j == m {
                mov.push((i, j));
            }
        }
    }

    // println!("{:?}", mov);

    let mut masu = vec![vec![-1; n]; n];
    masu[0][0] = 0;

    // masu = dps((0, 0), &mov, masu, n);
    masu = bps(vec![(0, 0); 1], &mov, masu, n);

    for y in 0..n {
        for x in 0..n {
            print!("{} ", masu[y][x]);
        }
        print!("\n");
    }
}
