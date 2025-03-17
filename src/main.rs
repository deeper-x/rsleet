mod probl35;
mod probl125;
mod probl88;
mod probl409;
mod probl20;
mod probl58;
mod probl121;

fn main() {
    let nums = vec![1,2,3,4,5];
    let target = 3;
    println!("Problem #35 - Result: {}", probl35::solution(nums, target));

    let in_str_125 = " ".to_string();
    println!("Problem #125 - Result: {}", probl125::solution(in_str_125));

    let in_str_409 = "abccccdd".to_string();
    println!("Problem #409 - Result: {}", probl409::solution(in_str_409));

    let in_str_20 = "{()[]}".to_string();
    println!("Problem #20 - Result: {}", probl20::solution(in_str_20));

    let in_str_58 = "Hello World".to_string();
    println!("Problem #58 - Result: {}", probl58::solution(in_str_58));

    let in_prices = vec![2,5,4,7,9,10];
    println!("Problem #121 - Result: {}", probl121::solution(in_prices));

    let mut in_nums1_88 = vec![1,2,3,0,0, 0];
    let mut in_nums2_88 = vec![2,5,6];

    probl88::solution(&mut in_nums1_88, 3, &mut in_nums2_88 , 3);
    println!("TODO - Problem #88 - Result: todo");

}
