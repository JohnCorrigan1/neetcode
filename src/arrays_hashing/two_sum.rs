use std::collections::HashMap;

fn main() {}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        if let Some(nice) = map.get(&(&target - nums[i])) {
            return vec![*nice, i as i32];
        }
        map.entry(nums[i]).or_insert(i as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn real() {
        let nums: Vec<i32> = vec![1, 2, 3, 5, 6, 7, 7];

        assert_eq!(two_sum(nums, 14), [5, 6]);
    }
}
