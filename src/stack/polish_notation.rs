fn main() {}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in &tokens {
        if let Ok(n) = token.parse::<i32>() {
            stack.push(n);
        } else {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();
            match token.as_str() {
                "+" => stack.push(n2 + n1),
                "-" => stack.push(n2 - n1),
                "*" => stack.push(n2 * n1),
                "/" => stack.push(n2 / n1),
                _ => panic!("Invalid token"),
            }
        }
    }

    stack.pop().unwrap()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        assert_eq!(eval_rpn(tokens), 9);
    }

    #[test]
    fn test_2() {
        let tokens = vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string(),
        ];
        assert_eq!(eval_rpn(tokens), 6);
    }

    #[test]
    fn test_3() {
        let tokens = vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string(),
        ];
        assert_eq!(eval_rpn(tokens), 22);
    }
}
