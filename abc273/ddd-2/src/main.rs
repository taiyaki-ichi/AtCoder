use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        rs:i64,
        cs:i64,
        n:usize,
        rc:[(i64,i64);n],
        q:usize,
        dl:[(char,i64);q]
    }

    let mut map = vec![vec![true; w]; h];
    for i in 0..n {
        map[rc[i].0 as usize - 1][rc[i].1 as usize - 1] = false;
    }

    let mut pos = (rs, cs);
    for i in 0..q {
        pos = update(pos, dl[i].0, dl[i].1, &map, w, h);
        println!("{} {}", pos.0, pos.1);
        // println!("");
    }
}

fn update(
    pos: (i64, i64),
    d: char,
    l: i64,
    map: &Vec<Vec<bool>>,
    w: usize,
    h: usize,
) -> (i64, i64) {
    if l == 0 {
        return pos;
    }

    // println!("{} {}", pos.0, pos.1);

    let dd = dir(d);
    let next = (pos.0 + dd.0, pos.1 + dd.1);
    if 0 < next.0 && next.0 <= h as i64 && 0 < next.1 && next.1 <= w as i64 {
        if map[next.0 as usize - 1][next.1 as usize - 1] {
            return update(next, d, l - 1, map, w, h);
        }
    }

    return pos;
}

fn dir(d: char) -> (i64, i64) {
    return match d {
        'L' => (0, -1),
        'R' => (0, 1),
        'U' => (-1, 0),
        'D' => (1, 0),
        _ => (0, 0),
    };
}
