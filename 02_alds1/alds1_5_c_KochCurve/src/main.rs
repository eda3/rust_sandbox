fn main() {
    let n: u8 = input();
    koch(n, 0.0, 0.0, 100.0, 0.0);
    println!("100.00000000 0.00000000");
}

fn input() -> u8 {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn koch(n: u8, x1: f64, y1: f64, x2: f64, y2: f64) {
    if n == 0 {
        println!("{:.8} {:.8}", x1, y1);
        return
    }

    let sx = (2. * x1 + 1. * x2) / 3.;
    let sy = (2. * y1 + 1. * y2) / 3.;

    let tx = (x1 + 2. * x2) / 3.;
    let ty = (y1 + 2. * y2) / 3.;

    let f = std::f64::consts::FRAC_PI_3;
    let ux = (tx - sx) * f.cos() - (ty - sy) * f.sin() + sx;
    let uy = (tx - sx) * f.sin() + (ty - sy) * f.cos() + sy;

    koch(n - 1, x1, y1, sx, sy);
    koch(n - 1, sx, sy, ux, uy);
    koch(n - 1, ux, uy, tx, ty);
    koch(n - 1, tx, ty, x2, y2);
}
