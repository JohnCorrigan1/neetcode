use std::collections::HashSet;
fn main() {
    let nums: Vec<i32> = vec![1, 1, 2, 2, 3, 3];
    println!("{}", contains_duplicate(nums));
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    nums.len() != set.len()
}

#[cfg(test)]
mod tests {

    use super::contains_duplicate;

    #[test]
    fn fake() {
        let nums: Vec<i32> = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(contains_duplicate(nums), true);
    }
    #[test]
    fn real() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(contains_duplicate(nums), false);
    }
}
