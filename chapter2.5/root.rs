fn root(arg: f64) -> f64 {
    let epsilon = 1e-10;
    let mut ans = 1.;
    loop {
        if (ans - (ans + arg / ans) / 2.).abs() < epsilon {
            break;
        } else {
            ans = (ans + arg / ans) / 2.;
        }

    }
    ans
}

fn main() {
    let arg = std::env::args().nth(1)
                .expect("Please provide only one argument")
                .parse::<f64>()
                .expect("Could not parse to an integer");

    println!("{}", root(arg));
    println!("{}", arg.sqrt());
}