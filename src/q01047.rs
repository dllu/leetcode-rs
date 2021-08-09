pub fn remove_duplicates(s: String) -> String {
    let mut s2: String = "?".to_string();
    s2.push_str(&s);
    s2.push_str(&("!".to_string()));
    let s = s2.as_bytes();
    let n = s.len();
    let mut next: Vec<usize> = (1..(n + 1)).collect();
    let mut prev: Vec<usize> = (0..1).chain(0..(n - 1)).collect();
    let mut i = 0;
    while next[i] < n {
        while i > 0 && next[i] < n && s[i] == s[next[i]] {
            let nex = next[next[i]];
            i = prev[i];
            prev[nex] = i;
            next[i] = nex;
        }
        i = next[i];
    }
    i = next[0];
    let mut ss = String::new();
    while s[i] as char != '!' {
        ss.push(s[i] as char);
        i = next[i];
    }
    ss
}

#[cfg(test)]
mod tests {
    #[test]
    fn q01047() {
        assert_eq!(super::remove_duplicates("abbaca".to_string()), "ca");
        assert_eq!(super::remove_duplicates("abb".to_string()), "a");
        assert_eq!(super::remove_duplicates("azxxzy".to_string()), "ay");
    }
}
