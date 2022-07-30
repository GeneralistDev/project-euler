// Pretty much fizzbuzz
fn main() {
    let limit = 1000;
    let mut sum = 0;

    for n in 1..limit {
        if n % 3 == 0 || n % 5 == 0 {
            sum = sum + n;
        }
    }

    println!("Sum: {}", sum);
}
