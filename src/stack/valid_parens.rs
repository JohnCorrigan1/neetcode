fn main() {}

pub fn valid_parentheses(s: String) -> bool {
    if s.len() < 2 {
        return false;
    }
    let mut stack: Vec<char> = vec![];
    let s: Vec<char> = s.chars().collect();

    for paren in s {
        match paren {
            '{' => stack.push(paren),
            '(' => stack.push(paren),
            '[' => stack.push(paren),
            '}' => {
                if stack.len() == 0 || stack[stack.len() - 1] != '{' {
                    return false;
                }
                stack.pop();
            }
            ')' => {
                if stack.len() == 0 || stack[stack.len() - 1] != '(' {
                    return false;
                }
                stack.pop();
            }
            ']' => {
                if stack.len() == 0 || stack[stack.len() - 1] != '[' {
                    return false;
                }
                stack.pop();
            }
            _ => panic!("ahhhhhhhh"),
        }
    }
    if stack.len() == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn real() {
        assert_eq!(valid_parentheses("({})[]".to_string()), true);
    }
}
