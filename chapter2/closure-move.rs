fn main() {
    let mut times = 2;
    {
        let mut borrow = |x| times += x;
        borrow(5);
    }
    println!("{}", times);

    let mut own = move |x| times += x;
    own(5);
    println!("{}", times);
}