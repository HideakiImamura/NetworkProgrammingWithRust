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
    let s = read::<String>().chars().collect::<Vec<char>>();
    let mut ans = 0;
    for i in 0..3 {
        if s[i] == '1' {
            ans += 1;
        }
    }
    println!("{}", ans);
}