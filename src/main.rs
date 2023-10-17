use num_bigint::BigUint;

fn main() {
    println!("Hello, world!");
}

struct Lychrel;

impl Lychrel {
    fn converges_at_iteration(n: u32, limit: u32) -> u32 {
        let n = BigUint::from(n);
        Lychrel::converge(&n, 0, limit)
    }

    fn converge(n: &BigUint, mut iteration: u32, limit: u32) -> u32 {
        if !Lychrel::is_palindrome(&n) && iteration < limit {
            let rev = Lychrel::reverse(&n);
            Lychrel::converge(&(n + rev), iteration + 1, limit)
        } else {
            iteration
        }
    }

    fn is_palindrome(n: &BigUint) -> bool {
        let s = n.to_string();
        let mut digits_iter = s.chars();
        let mut digits_reverse_iter = s.chars().rev();

        for i in 0..(s.len() / 2) {
            if digits_iter.next() != digits_reverse_iter.next() {
                return false;
            }
        }
        true
    }

    fn reverse(n: &BigUint) -> BigUint {           
        match n.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<BigUint>() {
                Ok(r) => r,
                Err(_) => panic!("Conversion error")
            }
    }

}




#[cfg(test)]
mod tests {

    use super::*;

    const LIMIT:u32 =1000;

    #[test]
    fn facts() {
        converges_at_iteration(1, 0);
        converges_at_iteration(2, 0);
        converges_at_iteration(10, 1);
        converges_at_iteration(11, 0);
        converges_at_iteration(19, 2);
        converges_at_iteration(78, 4);
        converges_at_iteration(89, 24);
        converges_at_iteration(187, 23);

        does_not_converge(196);
    }

    fn converges_at_iteration(n: u32, iteration: u32) {
        assert_eq!(iteration, Lychrel::converges_at_iteration(n, LIMIT));
    }

    fn does_not_converge(n: u32) {
        assert_eq!(LIMIT, Lychrel::converges_at_iteration(n, LIMIT));
    }

    #[test]
    fn palindromes() {
        is_palindrome(1);
        is_palindrome(11);
        is_palindrome(121);
        is_palindrome(12321);
        is_palindrome(1234321);
    }

    fn is_palindrome(n: u32) {
        let n = BigUint::from(n);
        assert!(Lychrel::is_palindrome(&n));
    }

    #[test]
    fn not_palindromes() {
        is_not_palindrome(10);
        is_not_palindrome(12331);
        is_not_palindrome(1243321);
    }

    fn is_not_palindrome(n: u32) {
        let n = BigUint::from(n);
        assert!(!Lychrel::is_palindrome(&n));
    }

    #[test]
    fn reversals() {
        reversed(0, 0);
        reversed(12, 21);
        reversed(123, 321);
        reversed(1234, 4321);
    }

    fn reversed(n: u32, r: u32) {
        let n = BigUint::from(n);
        assert_eq!(BigUint::from(r), Lychrel::reverse(&n));
    }
    

}