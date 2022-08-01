
// 1. fn is_palindrome(number)
// 1. multiples iterator

struct Multiples {
    digits: u64,
    left: u64,
    right: u64,
}

impl Multiples {
    fn new(digits: u64) -> Self {
        let left = (0..digits).map(|_| "9").collect::<String>().parse::<u64>().unwrap();
        let right = left;
        Self { digits, left, right }
    }
}

impl Iterator for Multiples {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let exponent = (self.digits - 1) as u32;
        let base_number = 10_u64.pow(exponent);

        let multiple = self.left * self.right;

        if self.right == base_number && self.left == base_number {
            return None;
        }

        if self.right > base_number {
            self.right -= 1;
        } else if self.left > base_number {
            self.left -= 1;
            self.right = self.left;
        }

        return Some(multiple);
    }
}

fn is_palindrome(n: &u64) -> bool {
    let string_representation = n.to_string();

    let mut a = 0;
    let mut b = string_representation.len() - 1;

    let mut palindrome = true;

    while a <= b {
        if string_representation.chars().nth(a).unwrap() != string_representation.chars().nth(b).unwrap() {
            palindrome = false;
            break;
        }

        a += 1;
        if b > 0 { b -= 1; }
    }

    return palindrome;
}

fn main() {
    let multiples = Multiples::new(3);

    let mut largest_palindrome: u64 = 0;

    for multiple in multiples {
        if is_palindrome(&multiple) && multiple > largest_palindrome{
            largest_palindrome = multiple;
        }
    }

    println!("Largest palindrome: {}", largest_palindrome);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(true, is_palindrome(&9009));
        assert_eq!(true, is_palindrome(&1));
        assert_eq!(true, is_palindrome(&90109));
        assert_eq!(false, is_palindrome(&23));
    }
}