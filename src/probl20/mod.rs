
pub fn solution(s: String) -> bool {
    let mut brackets: Vec<char> = Vec::new();
    
    for c in s.chars() {
        match c {
            '{' | '[' | '(' => brackets.push(c),
            '}' => if brackets.pop() != Some('{') { return false},
            ']' => if brackets.pop() != Some('[') { return false},
            ')' => if brackets.pop() != Some('(') {return false},
            _ => return false
        }
    }

    brackets.is_empty()    
}


#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    pub fn test_solution_1() {
        let got = solution("()".to_string());
        let expected = true;

        assert_eq!(got, expected);
    }


    #[test]
    pub fn test_solution_2() {
        let got = solution("()[]{}".to_string());
        let expected = true;

        assert_eq!(got, expected);
    }

    #[test]
    pub fn test_solution_3() {
        let got = solution("(]".to_string());
        let expected = false;

        assert_eq!(got, expected);
    }

    #[test]
    pub fn test_solution_4() {
        let got = solution("(]".to_string());
        let expected = false;

        assert_eq!(got, expected);
    }

    #[test]
    pub fn test_solution_5() {
        let got = solution("({[]})".to_string());
        let expected = true;

        assert_eq!(got, expected);
    }

    #[test]
    pub fn test_solution_6() {
        let got = solution("[{()}](){()}".to_string());
        let expected = true;

        assert_eq!(got, expected);
    }
}