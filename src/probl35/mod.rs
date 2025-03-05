pub fn solution(nums: Vec<i32>, target: i32 ) -> i32 {
    let mut res: i32 = 0;

    for (k, v) in nums.iter().enumerate() {
        if v >= &target {
            res = k as i32;
            break;
        }
        res = nums.len() as i32;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let nums = vec![1,2,3,4,5];
        let target = 3;
        
        let got = solution(nums, target);
        let expected = 2;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_2() {
        let nums = vec![1,3,5,6];
        let target = 7;

        let got = solution(nums, target);
        let expected = 4;

        assert_eq!(got, expected);

    }

    #[test]
    fn test_solution_3() {
        let nums = vec![1,3,5,6];
        let target = 2;

        let got = solution(nums, target);
        let expected = 1;

        assert_eq!(got, expected);
    }

    #[test]
    fn test_solution_4() {
        let nums = vec![1,3,5,6];
        let target = 2;

        let got = solution(nums, target);
        let expected = 1;

        assert_eq!(got, expected);
    }
}