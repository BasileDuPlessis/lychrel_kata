fn main() {
    println!("Hello, world!");
}

struct Lychrel;

impl Lychrel {
    fn converges_at_iteration(n: i32, _limit: i32) -> i32 {
        if !Lychrel::is_palindrome(n) {
            let r = Lychrel::reverse(n);
            let sum = r + n;
            if Lychrel::is_palindrome(sum) {
                1
            } else {
                2
            }
        } else {
            0
        }
    }

    fn is_palindrome(n: i32) -> bool {
        let digits = n.to_string();
        let mut digits_iter = digits.chars();
        let mut digits_reverse_iter = digits.chars().rev();

        for i in 0..(digits.len() / 2) {
            if digits_iter.next() != digits_reverse_iter.next() {
                return false;
            }
        }
       true
    }

    fn reverse(n: i32) -> i32 {
        let mut n = n;
        let mut r = 0;

        while n != 0 {
            let d = n % 10;
            n = n /10;
            r = r * 10 + d;
        }
        r
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    const LIMIT:i32 =1000;

    struct TestContext {
        limit: i32,
    }

    fn init_test_context() -> TestContext {
        TestContext {
            limit: 1000,
        }
    }

    #[test]
    fn facts() {
        let test_context = init_test_context();
        converges_at_iteration(1, 0);
        converges_at_iteration(2, 0);
        converges_at_iteration(10, 1);
        converges_at_iteration(11, 0);
        converges_at_iteration(19, 2);
        converges_at_iteration(78, 4);
    }

    fn converges_at_iteration(n: i32, iteration: i32) {
        assert_eq!(iteration, Lychrel::converges_at_iteration(n, LIMIT));
    }

    #[test]
    fn palindromes() {
        is_palindrome(1);
        is_palindrome(11);
        is_palindrome(121);
        is_palindrome(12321);
        is_palindrome(1234321);
    }

    fn is_palindrome(n: i32) {
        assert!(Lychrel::is_palindrome(n));
    }

    #[test]
    fn not_palindromes() {
        is_not_palindrome(10);
        is_not_palindrome(12331);
        is_not_palindrome(1243321);
    }

    fn is_not_palindrome(n: i32) {
        assert!(!Lychrel::is_palindrome(n));
    }

    #[test]
    fn reversals() {
        reversed(0, 0);
        reversed(12, 21);
        reversed(123, 321);
        reversed(1234, 4321);
    }

    fn reversed(n: i32, r: i32) {
        assert_eq!(r, Lychrel::reverse(n));
    }
    

}