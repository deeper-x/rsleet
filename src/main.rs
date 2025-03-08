mod probl35;
mod probl125;
mod probl409;

fn main() {
    let nums = vec![1,2,3,4,5];
    let target = 3;
    println!("Problem #35 - Result: {}", probl35::solution(nums, target));

    let in_str_125 = " ".to_string();
    println!("Problem #125 - Result: {}", probl125::solution(in_str_125));

    let in_str_409 = "abccccdd".to_string();
    println!("Problem #409 - Result: {}", probl409::solution(in_str_409));

}
