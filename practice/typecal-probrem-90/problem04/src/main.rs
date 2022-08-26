use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[usize;w];h]
    }

    let mut sum_w=vec![0;w];
    let mut sum_h=vec![0;h];

    for h_i in 0..h{
        for w_i in 0..w{
            sum_w[w_i]+=a[h_i][w_i];
            sum_h[h_i]+=a[h_i][w_i];
        }
    }

    for h_i in 0..h{
        for w_i in 0..w{
            print!("{} ",sum_w[w_i]+sum_h[h_i]-a[h_i][w_i]);
        }
        print!("\n");
    }
}
