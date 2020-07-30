fn main() {
    // 数列の長さ
    let n: usize = read()[0];

    // ランダムな数列
    let mut s: Vec<i32> = read();

    let count = merge_sort(&mut s, 0, n);
    println!("{}", s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    println!("{}", count);
}

// 一行読み込みを行う
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    buf.split_whitespace().flat_map(str::parse).collect()
}

fn merge_sort(a: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let mut count = 0;
    if left + 1 < right {
        let mid: usize = (left + right) / 2;
        count += merge_sort(a, left, mid);
        count += merge_sort(a, mid, right);
        count += merge(a, left, mid, right);
    }
    count
}

fn merge(a: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> usize {
    // リストの左側を作成
    let mut left_array = a[left..mid].to_vec();

    // リストの右側を作成
    let mut right_array = a[mid..right].to_vec();

    // 終端追加
    left_array.push(std::i32::MAX);
    right_array.push(std::i32::MAX);

    let mut i = 0;
    let mut j = 0;
    for k in left..right {
        if left_array[i] <= right_array[j] {
            a[k] = left_array[i];
            i += 1;
        } else {
            a[k] = right_array[j];
            j += 1;
        }
    }

    // iとjを合算が比較回数となる
    i + j
}