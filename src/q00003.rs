pub fn length_of_longest_substring(s: String) -> i32 {
    let mut latest = [-1; 255];
    let mut best = 0;
    let mut j = -1;
    for (i, &c) in s.as_bytes().iter().enumerate() {
        let i = i as i32;
        let cc = c as usize;
        if latest[cc] > j {
            j = latest[cc];
        }
        latest[cc] = i;
        if i - j > best {
            best = i - j;
        }
    }
    best as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn q00003() {
        assert_eq!(
            super::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(super::length_of_longest_substring("bbbbbbb".to_string()), 1);
        assert_eq!(super::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(super::length_of_longest_substring("".to_string()), 0);
        assert_eq!(super::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(super::length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(super::length_of_longest_substring("au".to_string()), 2);
    }
}
