fn read<T: std::str::FromStr>() -> T {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    n.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let a = read::<i32>();
    let b = read::<i32>();
    let c = read::<i32>();
    let x = read::<i32>() / 50;
    let mut ans = 0;

    for i in 0..a+1 {
        for j in 0..b+1 {
            for k in 0..c+1 {
                if 10 * i + 2 * j + k == x {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}