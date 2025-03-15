pub fn solution(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut x1 = m as isize - 1 ;
    let mut x2 = n as isize - 1;
    let mut x_t = (m + n) as isize - 1;

    while x2 >= 0 {
        if  x1 >= 0 && nums1[x1 as usize] > nums2[x2 as usize] {
            nums1[x_t as usize] = nums1[x1 as usize];
            x1 -= 1;
        } else {
            nums1[x_t as usize] = nums2[x2 as usize];
            x2 -= 1;
        }

        x_t -= 1
    }
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solution1() {
        let mut nums1 = vec![-1,0,0,3,3,3,0,0,0];
        let mut nums2 = vec![1,2,2];
        
        solution(&mut nums1, 6, &mut nums2, 3);
    }

    #[test]
    pub fn test_solution2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];

        solution(&mut nums1,1, &mut nums2, 0);
    }

    #[test]
    pub fn test_solution3() {
        let mut nums1 = vec![5,8,7,1,2,8,7,6,5,3,0,0,0,0];
        let mut nums2 = vec![5,7,2,3];

        solution(&mut nums1, 9, &mut nums2, 4)
    }
}