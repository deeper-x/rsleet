// SOLUTION 1:
// use std::collections::HashMap;

// pub fn solution(nums: Vec<i32>) ->  bool {
//     let mut dict: HashMap<i32, i32> = HashMap::new();

//     for i in nums {
//         let cur = dict.get(&i);

//         match cur {
//             Some(x) => {
//                 return true
//             },
//             None => {
//                 dict.insert(i, 1);
//             }
//         }
//     }
    
//     false
// }

// SOLUTION 2:
use std::collections::HashSet;

pub fn solution(nums: Vec<i32>) -> bool {
    let mut vals: HashSet<i32> = HashSet::new();

    for i in nums {
        if !vals.insert(i){
            return true
        }
    }

    false
}



#[cfg(test)]
mod tests  {
    use super::solution;


    #[test]
    pub fn test_solution1() {
        let in_list = vec![1,34,2,2,43,3,1,21,23];

        let res = solution(in_list);
        assert_eq!(res, true);
    }

    #[test]
    pub fn test_solution2() {
        let in_list = vec![1,2,3,1];
        let res = solution(in_list);
        assert_eq!(res, true);
    }

    #[test]
    pub fn test_solution3() {
        let in_list = vec![1,2,3,4];
        let res = solution(in_list);

        assert_eq!(res, false);
    }

    #[test]
    pub fn test_solution4() {
        let in_list = vec![1,1,1,3,3,4,3,2,4,2];
        let res = solution(in_list);

        assert_eq!(res, true);
    }

}