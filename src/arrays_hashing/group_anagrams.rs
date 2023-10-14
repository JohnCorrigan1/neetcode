use std::collections::HashMap;

fn main() {}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    if strs.len() < 2 {
        return vec![strs];
    }

    //let mut groups_map: HashMap<Vec<u8>, Vec<u16>> = HashMap::new();
    let mut groups_map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    let clone = strs.clone();

    let new: Vec<Vec<u8>> = clone.iter().map(|x| x.as_bytes().to_vec()).collect();

    for i in 0..new.len() {
        let mut new = new[i].clone();
        new.sort();
        if let Some(ana) = groups_map.get_mut(&new) {
            ana.push(strs[i].clone());
        } else {
            groups_map.insert(new.clone(), vec![strs[i].clone()]);
        }
    }

    groups_map.into_iter().map(|x| x.1).collect()
    //let mut groups: Vec<Vec<String>> = vec![];

    //    for words in groups_map {
    //groups.push(words.1.iter().map(|x| strs[*x as usize].clone()).collect())
    //}
    // groups
    //
}

#[cfg(test)]
mod tests {

    use super::group_anagrams;

    #[test]
    fn real() {
        let strs: Vec<&str> = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let ans: Vec<Vec<&str>> = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        let strs: Vec<String> = strs.iter().map(|x| x.to_string()).collect();
        let ans: Vec<Vec<String>> = ans
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
            .collect();

        assert_eq!(group_anagrams(strs), ans);
    }
}
