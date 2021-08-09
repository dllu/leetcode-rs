pub fn is_number(s: String) -> bool {
    if s == "inf" || s == "-inf" || s == "+inf" || s == "nan" {
        return false;
    }
    s.parse::<f64>().is_ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn q00065() {
        assert!(super::is_number("0".to_string()));
        assert!(!super::is_number("e".to_string()));
        assert!(!super::is_number(".".to_string()));
        assert!(super::is_number(".1".to_string()));
        assert!(!super::is_number("inf".to_string()));
    }
}
