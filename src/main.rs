pub mod guessing_game;
pub mod test_bindings;

fn main() {
    println!("Hello, world!");
    guessing_game::main();
    test_bindings::main();
}
