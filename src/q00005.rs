/// len[2*i] length of longest odd palindrome at i
/// len[2*i + 1] length of longest even palindrome at i
pub fn longest_palindrome(s: String) -> String {
    let s = s.as_bytes();
    let n = s.len() as i64;
    let mut len = vec![0; (2 * n) as usize];
    len[0] = 1;
    len[1] = 0;
    let mut i: i64 = 1;
    while i + 1 < 2 * n {
        let pal = len[i as usize];
        let mut left = (i - pal - 1) / 2;
        let mut right = (i + pal + 1) / 2;

        while 0 <= left && right < n && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
            len[i as usize] += 2;
        }
        let mut d = 1;
        while len[(i - d) as usize] < len[i as usize] - d {
            len[(i + d) as usize] = len[(i - d) as usize];
            d += 1;
        }
        len[(i + d) as usize] = len[i as usize] - d;
        i += d;
    }
    let mut best = 0;
    let mut besti = 0;
    for (i, &pal) in len.iter().enumerate() {
        if pal > best {
            besti = i;
            best = pal;
        }
    }
    let start = (besti + 1 - best as usize) / 2;
    let end = start + best as usize;
    std::str::from_utf8(&s[start..end]).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn q00005() {
        assert_eq!(
            super::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            super::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            super::longest_palindrome("ccc".to_string()),
            "ccc".to_string()
        );
    }
}
