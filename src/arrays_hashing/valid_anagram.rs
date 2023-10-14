use std::collections::HashMap;

fn main() {}

pub fn valid_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut s_map: HashMap<u8, u16> = HashMap::new();
    let mut t_map: HashMap<u8, u16> = HashMap::new();

    for i in 0..s.len() {
        let letter = s_map.entry(s[i]).or_insert(0);
        let letter2 = t_map.entry(t[i]).or_insert(0);
        *letter += 1;
        *letter2 += 1;
    }

    if s_map.len() != t_map.len() {
        return false;
    }

    for key in s_map {
        if let Some(tv) = t_map.get(&key.0) {
            if &key.1 != tv {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {

    use super::valid_anagram;

    #[test]
    fn real() {
        let s: String = String::from("abs");
        let t: String = String::from("asb");
        assert_eq!(valid_anagram(s, t), true);
    }

    #[test]
    fn fake() {
        let s: String = String::from("acs");
        let t: String = String::from("asb");
        assert_eq!(valid_anagram(s, t), false);
    }
}
