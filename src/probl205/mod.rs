use std::collections::HashMap;

pub fn solution(s: String, t: String) -> bool {
    let mut res = false;
    let mut left: HashMap<char, char> = HashMap::new();
    let mut right: HashMap<char, char> = HashMap::new();
    let mut counter = 0;

    for c1 in s.chars() {
        let c2 = t.chars().nth(counter).unwrap();

        if !left.contains_key(&c1) {
            left.insert(c1, c2);
        } else {
            let cur_l = left.get(&c1).unwrap();
            if cur_l != &c2 {
                return res;
            }
        }

        if !right.contains_key(&c2) {
            right.insert(c2, c1);
        } else {
            let cur_r = right.get(&c2).unwrap();
            if cur_r != &c1 {
                return res;
            }
        }

        counter += 1;
    }

    res = true;
    res
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_solution_1() {
        let got = solution("paper".to_string(), "title".to_string());
        let expected = true;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_2() {
        let got = solution("badc".to_string(), "baba".to_string());
        let expected = false;

        assert_eq!(got, expected);
    }

    #[test]
    fn tet_solution_3() {
        let got = solution("egg".to_string(), "add".to_string());
        let expected = true;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_4() {
        let got = solution("badc".to_string(), "baba".to_string());
        let expected = false;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_5() {
        let got = solution("foo".to_string(), "bar".to_string());
        let expected = false;

        assert_eq!(got, expected);
    }
}
