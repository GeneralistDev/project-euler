fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = a + b;

    let mut sum_even = 2;

    while c < 4_000_000 {
        if c % 2 == 0 {
            sum_even = sum_even + c;
        }
        a = b;
        b = c;
        c = a + b;
    }

    println!("Sum: {}", sum_even);
}
