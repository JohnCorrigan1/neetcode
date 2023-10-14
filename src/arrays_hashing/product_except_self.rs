fn main() {
    product_except_self(vec![1, 2, 3, 4]);
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![nums[0]];

    let mut left = nums[0];

    for i in 1..nums.len() {
        result.push(left);
        left *= nums[i];
    }

    let mut right = nums[nums.len() - 1];

    for i in (0..nums.len() - 1).rev() {
        result[i] *= right;
        right *= nums[i];
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}
