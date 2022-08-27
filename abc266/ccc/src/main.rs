use proconio::input;

fn main() {
    input! {
        ax:i64,
        ay:i64,
        bx:i64,
        by:i64,
        cx:i64,
        cy:i64,
        dx:i64,
        dy:i64,
    }

    let dot = |a1, a2, b1, b2| a1 * b2 - a2 * b1;

    let a = dot(dx - ax, dy - ay, ax - bx, ay - by);
    let b = dot(ax - bx, ay - by, bx - cx, by - cy);
    let c = dot(bx - cx, by - cy, cx - dx, cy - dy);
    let d = dot(cx - dx, cy - dy, dx - ax, dy - ay);

    println!(
        "{}",
        if a >= 0 && b >= 0 && c >= 0 && d >= 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
