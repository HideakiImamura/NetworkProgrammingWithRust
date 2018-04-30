fn parse_int(s: String) -> u64 {
    return s.parse::<u64>().expect("Could not parse as interger")
}

fn main() {
    let _ = parse_int("1".to_owned());
    let _ = parse_int("abcd".to_owned());
}
