pub fn solution(s: String) -> i32 {
    let container: Vec<&str> = s.split_whitespace().collect();
    let last = container[container.len()-1];

    last.len() as i32
}


#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    pub fn solution_1() {
        let got = solution("Hello World".to_string());
        let expected = 5;

        assert_eq!(got, expected)
    }

    #[test]
    pub fn solution_2() {
        let got = solution("   fly me   to   the moon  ".to_string());
        let expected = 4;

        assert_eq!(got, expected);
    }

    #[test]
    pub fn solution_3() {
        let got = solution("luffy is still joyboy".to_string());
        let expected = 6;

        assert_eq!(got, expected);
    }
}