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

fn solve(n: u32, mut a_vec: Vec<u64>) -> u64 {
    let mut b = true;
    for i in 0..n {
        if a_vec[i as usize] % 2 != 0 {
            b = false;
            break;
        } else {
            a_vec[i as usize] /= 2;
        }
    }
    if b {
        1 + solve(n, a_vec)
    } else {
        0
    }
}

fn main() {
    let n = read::<u32>();
    let a_vec = read_vec::<u64>();

    println!("{}", solve(n, a_vec));
}