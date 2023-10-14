use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![1, 1, 2, 2, 3, 4, 5, 6, 7, 7, 7];
    let k = 2;
    top_k_frequent(nums, k);
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts: HashMap<i32, u16> = HashMap::new();

    for num in nums {
        if let Some(count) = counts.get_mut(&num) {
            *count += 1;
        } else {
            counts.insert(num, 1);
        }
    }

    let mut result: Vec<(i32, u16)> = counts.into_iter().map(|x| x).collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result.iter().take(k as usize).map(|&(x, _)| x).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn real() {
        let elements: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
        let k = 2;

        let result = top_k_frequent(elements, k);
        assert_eq!(result, vec![1, 2]);
    }
}
