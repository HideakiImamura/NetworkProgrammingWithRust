fn main() {
    let arg = std::env::args().nth(1)
                .expect("Please provide only one argument")
                .parse::<u64>()
                .expect("Could not parse to an integer");
    for i in 1..arg+1 {
        if i % 3 == 0 {
            if i % 5 == 0 {
                println!("FizzBuzz");
            } else {
                println!("Buzz");
            }
        } else {
            if i % 5 == 0 {
                println!("Fizz");
            } else {
                println!("{}", i);
            }
        }
    }
}