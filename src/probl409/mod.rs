use std::collections::HashMap;

pub fn solution(s: String) -> i32{
    let mut dic_count: HashMap<char, i32> = HashMap::new();
    let mut tot_pal = 0;
    let mut has_odd = false;
    let s_len = s.len() as i32;

    for c in s.chars() {
        *dic_count.entry(c).or_insert(0) += 1;
    }

    for (_, v) in &dic_count {
        if v % 2 != 0 && *v > 2 && *v == s_len{
            tot_pal = *v;
            break;
        } else if *v == 1 {
            has_odd = true
        } else if v % 2 != 0 && *v > 2 {
            tot_pal += v-1;
            has_odd = true
        } else if v % 2 == 0 && *v > 2 {
            tot_pal += *v;
        } else if *v == 2 {
            tot_pal += 2
        }
    }

    if has_odd {
        tot_pal += 1;
    }

    tot_pal
}


#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    pub fn test_solution_1() {
        let in_str = "ccc".to_string();
        let got = solution(in_str);
        let expected = 3;

        assert_eq!(got, expected);
    }

    #[test]
    pub fn test_solution_2() {
        let in_str = "bananas".to_string();
        let got = solution(in_str);
        let expected = 5;

        assert_eq!(got, expected);
    }

    #[test]
    pub fn test_solution_3() {
        let in_str = "abccccdd".to_string();
        let got = solution(in_str);
        let expected = 7;

        assert_eq!(got, expected);
    }
}