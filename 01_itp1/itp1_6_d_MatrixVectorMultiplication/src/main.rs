fn main() {
    let v: Vec<usize> = r();
    let (n, m) = (v[0], v[1]);

    let mut a: Vec<Vec<_>> = vec!();
    for _ in 0..n {
        let v: Vec<u32> = r();
        a.push(v);
    }

    let mut b: Vec<_> = vec!();
    for _ in 0..m {
        let v: Vec<u32> = r();
        b.push(v[0]);
    }

    for i in 0..n {
        let mut res = 0;
        for aa in b.iter().zip(a[i].iter()).map(|(x, y)| x * y) {
            res += aa
        }
        println!("{:?}", res);
    }
}

fn r<T: std::str::FromStr>() -> Vec<T> {
    let mut b = String::new();
    let s = std::io::stdin();
    s.read_line(&mut b).unwrap();
    b.split_whitespace().flat_map(str::parse).collect()
}