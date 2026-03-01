
/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
     
    let mut x: u32 = 0;

    loop {
        if n == 1 {
            x += 1;
            break;
        } else {
            if n % 2 == 0 {
                n = dbg!(n / 2);
                x += 1;
            } else {
                // collatz_length(3*n + 1)
                n = dbg!(3 * n + 1);
                x += 1;
            }
        }
    }
    dbg!(x);
    x
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
