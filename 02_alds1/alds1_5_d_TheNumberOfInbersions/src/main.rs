fn main() {
    // 数列の個数
    let n: usize = read()[0];

    // 数列取得
    let a: Vec<usize> = read();

    // 数列の順番格納用
    let mut b: Vec<(usize, usize)> = vec![];
    for (i, &item) in a.iter().enumerate(){
        b.push((item, i+1));
    }
    b.sort();
    println!("{:?}", b);

    let mut c: Vec<usize> = vec![];
    for i in b.iter(){
        // c.push(i[0][0])
        (x, y) = i;
        println!("{:?}", y);

    }
    println!("{:?}", c);

}

fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    buf.split_whitespace().flat_map(str::parse).collect()
}
