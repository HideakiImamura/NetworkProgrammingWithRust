fn main() {
    let arg = std::env::args().nth(1).expect("Please provide only one argument");
    let arg = arg.parse::<u64>().expect("Could not parse to an integer");
    for i in 1..arg+1 {
        println!("Hello World! {}", i);
    }
}