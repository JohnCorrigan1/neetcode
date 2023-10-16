use std::collections::HashSet;

fn main() {}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let set: HashSet<i32> = nums.into_iter().collect();
    let mut curr = 1;

    for num in &set {
        if set.contains(&(num - 1)) {
            continue;
        } else if set.contains(&(num + 1)) {
            let mut count = 2;
            loop {
                if set.contains(&(num + count)) {
                    count += 1;
                } else {
                    if count > curr {
                        curr = count;
                    }
                    break;
                }
            }
        }
    }

    curr
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_128() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
