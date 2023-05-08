pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn q00009() {
        assert!(super::is_palindrome(12321));
        assert!(!super::is_palindrome(12345));
    }
}
