// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
// You must write an algorithm with O(log n) runtime complexity.

// Example 1:
// Input: nums = [1,3,5,6], target = 5
// Output: 2

// Example 2:
// Input: nums = [1,3,5,6], target = 2
// Output: 1

// Example 3:
// Input: nums = [1,3,5,6], target = 7
// Output: 4
 
// Constraints:
// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums contains distinct values sorted in ascending order.
// -104 <= target <= 104


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