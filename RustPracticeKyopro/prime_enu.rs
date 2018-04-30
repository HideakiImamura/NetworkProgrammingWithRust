fn prime_enumerate(arg: u64) -> Vec<u64> {
    let mut ans: Vec<u64> = (2..arg+1).collect();
    let root = (arg as f64).sqrt() as u64;
    for m in 2..root + 1 {
        for i in 0..ans.len() {
            if ans[i] % m == 0 && ans[i] / m != 1 {
                ans[i] = 0;
            }
        }
    }
    ans.sort();
    loop {
        if ans[0] == 0 {
            ans.remove(0);
        } else {
            break;
        }
    }
    ans
}

fn main() {
    let arg = std::env::args().nth(1)
                .expect("Please provide only one argument")
                .parse::<u64>()
                .expect("Could not parse to an integer");

    let ps = prime_enumerate(arg);

    println!("{:?}", ps);
}