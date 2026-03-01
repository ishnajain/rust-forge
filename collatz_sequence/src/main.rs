/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: u32) -> u32 {
    let mut x: u32 = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        x += 1;
    }
    x
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
