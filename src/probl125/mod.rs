pub fn solution(s: String) -> bool {
    let mut res = true;
    let mut s_clean = String::new();

    if s.len() <= 1 {
        return res;
    }

    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            s_clean.push(c);
        }
    }

    let s_len = s_clean.len();
    let mut last_idx = s_len-1;
    let half = s_len / 2;
    
    for (k, c) in s_clean.chars().enumerate() {
        if k >= half {
            break;
        }

        let cur_r = s_clean.chars().nth(last_idx).unwrap();
        
        if !c.eq_ignore_ascii_case(&cur_r) {
            res = false;
            break;
        }

        last_idx -= 1;
    }
    
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let got = solution("A man, a plan, a canal: Panama".to_string());
        let expected = true;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_2() {
        let got = solution("race a car".to_string());
        let expected = false;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_3() {
        let got = solution("0P".to_string());
        let expected = false;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_4() {
        let got = solution(" ".to_string());
        let expected = true;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_5() {
        let got = solution("a normal string".to_string());
        let expected = false;

        assert_eq!(got, expected);
    }
}