fn is_palindrome(n: u32) -> bool {
    let original = n;
    let mut reversed = 0;
    let mut num = n;

    while num > 0 {
        let digit = num % 10;
        reversed = reversed * 10 + digit;
        num /= 10;
    }

    original == reversed
}

fn main() {
    let test_cases = [121, 123, 12321, 456, 909];

    for &num in &test_cases {
        if is_palindrome(num) {
            println!("{} is a palindrome", num);
        } else {
            println!("{} is not a palindrome", num);
        }
    }
}
