use proconio::input;

fn main() {
    input! {
        a:[usize;5],
    }

    let hoge=a.binary_search(&1);

    match hoge {
        Ok([q,w])=>println!("{:?}",a),
        _=>println!("aaa"),
        
    }

}
