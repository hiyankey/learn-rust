pub fn main() {
    let mut str = "Hello world!";
    str = "Hello dev!";
    println!("{}", str);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let spaces = "  ";

    let _spaces = spaces.len();
}
